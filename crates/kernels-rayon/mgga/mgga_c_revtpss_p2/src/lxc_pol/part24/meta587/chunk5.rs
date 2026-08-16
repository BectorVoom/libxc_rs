//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1829/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1829(t124: f64, t1370: f64, t47337: f64, t49087: f64, t49090: f64, t49105: f64, t74638: f64, t74641: f64, t74677: f64, t74682: f64, t74711: f64, t74714: f64, t74717: f64, t800: f64, t86240: f64, t86244: f64, t86256: f64, t86260: f64, t86264: f64, t86274: f64, t91826: f64) -> f64 {
    let t92216 = -0.12196800674228478774e-3_f64 * t74638 - 0.27107389498472794074e-4_f64 * t74641 + 0.16006300097412701803e-1_f64 * t86240 + 0.28582678745379824648e-4_f64 * t86244 - 0.73180804045370872643e-3_f64 * t49087 + 0.13011546959266941156e-2_f64 * t49090 + 0.18071592998981862717e-5_f64 * t49105 - 0.50820002809285328224e-4_f64 * t86256 - 0.34299214494455789577e-3_f64 * t86260 - 0.34299214494455789577e-3_f64 * t86264 + 0.54214778996945588149e-4_f64 * t74677 - t1370 * t800 * t124 * t91826 / 48.0_f64 + 35.0_f64 / 12.0_f64 * t74682 + 0.6098400337114239387e-4_f64 * t74711 - 0.30492001685571196935e-3_f64 * t74714 + t47337 - 35.0_f64 / 36.0_f64 * t74717 + 0.68598428988911579156e-3_f64 * t86274;
    t92216
}
