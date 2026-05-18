//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1231/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1231<F: Float>(t1318: F, t2151: F, t2014: F, t684: F, t8562: F, t3146: F, t6469: F, t23622: F, t3151: F, t686: F, t19643: F, t1346: F, t2234: F) -> (F, F, F, F, F, F) {
    let t24455 = t2151 * t1318;
    let t24461 = t684 * t2014 * t8562;
    let t24464 = t684 * t6469 * t3146;
    let t24468 = t684 * t23622 * t686 * t3151;
    let t24480 = F::new(24.0) * t19643;
    let t24497 = t2234 * t1346;
    (t24455, t24461, t24464, t24468, t24480, t24497)
}
