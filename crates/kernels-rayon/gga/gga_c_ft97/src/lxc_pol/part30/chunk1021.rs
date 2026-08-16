//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1021/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1021(t140892: f64, t27616: f64, t6804: f64, t27733: f64, t33371: f64, t108517: f64, t1095: f64, t140885: f64, t140945: f64, t140960: f64, t141089: f64, t141096: f64, t141097: f64, t141117: f64, t141123: f64, t141125: f64, t141176: f64, t218: f64, t22532: f64, t24340: f64, t27533: f64, t27552: f64, t27561: f64, t27565: f64, t27647: f64, t27686: f64, t27689: f64, t33375: f64, t36792: f64, t36835: f64, t79641: f64) -> f64 {
    let t150471 = t27616 * t140892 * t6804;
    let t150486 = t27733 * t33371;
    let t150494 = -0.11738898233082762228e-1_f64 * t140945 + 0.1136661281381420225e-5_f64 * t141089 * t140885 * t27561 + 0.51074886703703703704e-1_f64 * t141096 * t141097 * t27647 - 0.13200366700519885118e-5_f64 * t150471 + 0.26086440517961693841e-2_f64 * t140960 - 0.17816121467177433867e-3_f64 * t141176 * t27686 - 0.59346127734643676855e-4_f64 * t108517 * t36835 * t22532 * t27689 + 0.79202200203119310706e-5_f64 * t141117 * t36792 * t27552 - 0.45497819271775541929e-4_f64 * t141123 * t141125 * t27533 + 0.15322466011111111111e0_f64 * t150486 * t33375 - 0.24041029937711879614e-5_f64 * t79641 * t27565 * t24340 * t218 * t1095;
    t150494
}
