//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 942/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk942<F: Float>(t2014: F, t3283: F, t684: F, t1318: F, t763: F, t675: F, t2002: F, t3282: F, t2028: F, t1243: F, t6469: F, t1240: F, t2011: F) -> (F, F, F, F, F, F, F) {
    let t8560 = t684 * t2014 * t3283 / F::cast_from(96.0_f64);
    let t8561 = t763 * t1318;
    let t8562 = t8561 * t675;
    let t8566 = t3282 * t2002;
    let t8570 = t3282 * t2028;
    let t8575 = t684 * t6469 * t1243;
    let t8577 = t1240 * t2011;
    (t8560, t8561, t8562, t8566, t8570, t8575, t8577)
}
