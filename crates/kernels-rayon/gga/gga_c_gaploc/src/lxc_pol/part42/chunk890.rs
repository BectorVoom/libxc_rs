//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 890/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk890(t11016: f64, t11798: f64, t36477: f64, t955: f64, t3470: f64, t37057: f64, t36512: f64, t41339: f64, t10742: f64, t10867: f64, t900: f64, t44130: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45933 = 0.7150097990370085334e0_f64 * t11798 * t11016;
    let t45939 = 0.23833659967900284446e0_f64 * t955 * t36477;
    let t45946 = 0.10725146985555128001e1_f64 * t37057 * t3470;
    let t45947 = t36512 * t41339;
    let t45950 = t10867 * t900 * t10742;
    let t45953 = 0.17875244975925213335e0_f64 * t44130;
    (t45933, t45939, t45946, t45947, t45950, t45953)
}
