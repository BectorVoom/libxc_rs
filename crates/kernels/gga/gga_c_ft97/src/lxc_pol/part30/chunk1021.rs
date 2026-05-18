//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1021/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1021<F: Float>(t140892: F, t27616: F, t6804: F, t27733: F, t33371: F, t108517: F, t1095: F, t140885: F, t140945: F, t140960: F, t141089: F, t141096: F, t141097: F, t141117: F, t141123: F, t141125: F, t141176: F, t218: F, t22532: F, t24340: F, t27533: F, t27552: F, t27561: F, t27565: F, t27647: F, t27686: F, t27689: F, t33375: F, t36792: F, t36835: F, t79641: F) -> F {
    let t150471 = t27616 * t140892 * t6804;
    let t150486 = t27733 * t33371;
    let t150494 = -F::new(0.11738898233082762228e-1) * t140945 + F::new(0.1136661281381420225e-5) * t141089 * t140885 * t27561 + F::new(0.51074886703703703704e-1) * t141096 * t141097 * t27647 - F::new(0.13200366700519885118e-5) * t150471 + F::new(0.26086440517961693841e-2) * t140960 - F::new(0.17816121467177433867e-3) * t141176 * t27686 - F::new(0.59346127734643676855e-4) * t108517 * t36835 * t22532 * t27689 + F::new(0.79202200203119310706e-5) * t141117 * t36792 * t27552 - F::new(0.45497819271775541929e-4) * t141123 * t141125 * t27533 + F::new(0.15322466011111111111e0) * t150486 * t33375 - F::new(0.24041029937711879614e-5) * t79641 * t27565 * t24340 * t218 * t1095;
    t150494
}
