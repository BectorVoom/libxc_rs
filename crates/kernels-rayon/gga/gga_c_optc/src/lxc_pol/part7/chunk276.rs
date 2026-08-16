//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 276/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk276(t828: f64, t836: f64, t837: f64, t845: f64, t278: f64, t280: f64, rho0: f64) -> (f64, f64, f64, f64) {
    let t847 = t828 * t836 * t837;
    let t849 = 0.58482233974552040708e0_f64 * t845 * t847;
    let t850 = t278 * rho0;
    let t852 = 1.0_f64 / t280 / t850;
    (t847, t849, t850, t852)
}
