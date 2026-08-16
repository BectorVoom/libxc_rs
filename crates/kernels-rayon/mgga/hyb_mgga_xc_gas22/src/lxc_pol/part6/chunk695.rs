//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 695/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk695(t3517: f64, t967: f64, t2521: f64, t2457: f64, t2527: f64, t3461: f64, t3472: f64, t1414: f64, t978: f64) -> (f64, f64, f64, f64) {
    let t3518 = t3517 * t967;
    let t3520 = 0.16081979498692535067e2_f64 * t2521 * t3518;
    let t3524 = t2527 - 0.17123333333333333333e-1_f64 * t2457 - 0.17123333333333333333e-1_f64 * t3461 + 0.5137e-1_f64 * t3472;
    let t3527 = t1414 * t978;
    (t3518, t3520, t3524, t3527)
}
