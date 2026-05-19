//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 154/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk154<F: Float>(t439: F, t442: F, t445: F, t448: F) -> (F, F, F) {
    let t475 = F::new(0.705945e1) * t442 + F::new(0.1549425e1) * t439 + F::new(0.420775e0) * t445 + F::new(0.1562925e0) * t448;
    let t478 = F::new(1.0) + F::cast_from(0.32163958997385070134e2_f64) / t475;
    let t479 = F::ln(t478);
    (t475, t478, t479)
}
