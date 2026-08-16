//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2025/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2025(t91394: f64, t91398: f64, t91402: f64, t91404: f64, t80920: f64, t80922: f64, t80940: f64, t80943: f64, t80959: f64, t80989: f64, t80992: f64, t80998: f64, t81007: f64, t84555: f64, t84558: f64, t91400: f64, t91413: f64, t91416: f64) -> f64 {
    let t93757 = 119.0_f64 / 3456.0_f64 * t91394;
    let t93760 = 35.0_f64 / 108.0_f64 * t91398;
    let t93762 = 7.0_f64 / 36.0_f64 * t91402;
    let t93763 = 0.33913115119077928316e-1_f64 * t91404;
    let t93773 = -t93757 + 0.28260929265898273597e-2_f64 * t80920 + 0.28260929265898273597e-2_f64 * t80922 - t93760 - 0.13565246047631171326e0_f64 * t91400 + t93762 + t93763 - 0.45217486825437237756e-1_f64 * t80940 - 0.56521858531796547194e-2_f64 * t80943 - t84555 - 0.33913115119077928316e-1_f64 * t80959 + t84558 + 7.0_f64 / 1152.0_f64 * t80989 + 7.0_f64 / 576.0_f64 * t80992 - 7.0_f64 / 576.0_f64 * t80998 + 7.0_f64 / 1152.0_f64 * t81007 + t91413 / 96.0_f64 + t91416 / 768.0_f64;
    t93773
}
