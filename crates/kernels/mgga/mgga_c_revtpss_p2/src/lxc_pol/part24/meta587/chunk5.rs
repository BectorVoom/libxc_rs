//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1829/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1829<F: Float>(t124: F, t1370: F, t47337: F, t49087: F, t49090: F, t49105: F, t74638: F, t74641: F, t74677: F, t74682: F, t74711: F, t74714: F, t74717: F, t800: F, t86240: F, t86244: F, t86256: F, t86260: F, t86264: F, t86274: F, t91826: F) -> F {
    let t92216 = -F::cast_from(0.12196800674228478774e-3_f64) * t74638 - F::cast_from(0.27107389498472794074e-4_f64) * t74641 + F::cast_from(0.16006300097412701803e-1_f64) * t86240 + F::cast_from(0.28582678745379824648e-4_f64) * t86244 - F::cast_from(0.73180804045370872643e-3_f64) * t49087 + F::cast_from(0.13011546959266941156e-2_f64) * t49090 + F::cast_from(0.18071592998981862717e-5_f64) * t49105 - F::cast_from(0.50820002809285328224e-4_f64) * t86256 - F::cast_from(0.34299214494455789577e-3_f64) * t86260 - F::cast_from(0.34299214494455789577e-3_f64) * t86264 + F::cast_from(0.54214778996945588149e-4_f64) * t74677 - t1370 * t800 * t124 * t91826 / F::new(48.0) + F::new(35.0) / F::new(12.0) * t74682 + F::cast_from(0.6098400337114239387e-4_f64) * t74711 - F::cast_from(0.30492001685571196935e-3_f64) * t74714 + t47337 - F::new(35.0) / F::new(36.0) * t74717 + F::cast_from(0.68598428988911579156e-3_f64) * t86274;
    t92216
}
