//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1049/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1049(t43288: f64, t43289: f64, t43295: f64, t43300: f64, t43312: f64, t43315: f64, t43321: f64, t43325: f64, t43335: f64, t43339: f64, t47749: f64, t47752: f64, t47755: f64, t47758: f64, t47764: f64, t47766: f64, t47768: f64, t47772: f64) -> f64 {
    let t51054 = t43288 - t43289 + t43295 + t43300 - 0.10766736722199113997e0_f64 * t47749 + 0.15381052460284448567e-1_f64 * t47752 + 0.15381052460284448567e-1_f64 * t47755 + 0.15381052460284448567e-1_f64 * t47758 + t47764 - t47766 - 0.64087718584518535698e-3_f64 * t47768 - 0.64087718584518535698e-3_f64 * t47772 + t43312 + t43315 + t43321 + t43325 + t43335 - t43339;
    t51054
}
