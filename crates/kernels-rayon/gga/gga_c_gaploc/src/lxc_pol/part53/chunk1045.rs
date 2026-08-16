//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1045/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1045(t40775: f64, t43166: f64, t43168: f64, t43173: f64, t43179: f64, t43182: f64, t43185: f64, t43189: f64, t43195: f64, t47673: f64, t47677: f64, t47681: f64, t47685: f64, t47687: f64, t47690: f64, t47693: f64, t47696: f64, t47702: f64) -> f64 {
    let t51029 = -0.15381052460284448567e-1_f64 * t47673 + 0.15381052460284448567e-1_f64 * t47677 + 0.18457262952341338281e0_f64 * t47681 - 0.92286314761706691402e-1_f64 * t47685 + 0.64087718584518535698e-3_f64 * t47687 + 0.64087718584518535698e-3_f64 * t47690 - 0.34180116578409885704e-2_f64 * t47693 + 0.51270174867614828558e-2_f64 * t47696 - t43166 - t43168 + t43173 - t43179 + t43182 + t43185 - t43189 - 0.19226315575355560709e-2_f64 * t40775 - t43195 - 0.85450291446024714264e-3_f64 * t47702;
    t51029
}
