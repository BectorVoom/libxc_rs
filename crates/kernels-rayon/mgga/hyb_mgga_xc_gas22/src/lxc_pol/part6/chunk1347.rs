//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1347/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1347(t24699: f64, t3321: f64, t24709: f64, t3357: f64, t8739: f64, t8923: f64, t8618: f64, t8906: f64, t10638: f64, t6574: f64, t10641: f64, t6497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29398 = 8.0_f64 * t24699 * t3321;
    let t29400 = 0.64327917994770140268e2_f64 * t24709 * t3357;
    let t29402 = 8.0_f64 * t8923 * t8739;
    let t29404 = 0.64327917994770140268e2_f64 * t8906 * t8618;
    let t29406 = 12.0_f64 * t6574 * t10638;
    let t29408 = 8.0_f64 * t6497 * t10641;
    (t29398, t29400, t29402, t29404, t29406, t29408)
}
