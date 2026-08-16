//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 313/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk313(t1089: f64, t483: f64, t1035: f64, t1037: f64, t1041: f64, t1044: f64) -> (f64, f64) {
    let t1090 = t483 * t1089;
    let t1095 = -0.86308333333333333334e0_f64 * t1035 - 0.301925e0_f64 * t1037 - 0.5501625e-1_f64 * t1041 - 0.82785e-1_f64 * t1044;
    (t1090, t1095)
}
