//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 899/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk899<F: Float>(t43288: F, t43289: F, t43295: F, t43300: F, t43312: F, t43315: F, t43321: F, t43325: F, t43335: F, t43339: F, t47749: F, t47752: F, t47755: F, t47758: F, t47764: F, t47766: F, t47768: F, t47772: F) -> (F,) {
    let t51054 = t43288 - t43289 + t43295 + t43300 - 0.10766736722199113997e0 * t47749 + 0.15381052460284448567e-1 * t47752 + 0.15381052460284448567e-1 * t47755 + 0.15381052460284448567e-1 * t47758 + t47764 - t47766 - 0.64087718584518535698e-3 * t47768 - 0.64087718584518535698e-3 * t47772 + t43312 + t43315 + t43321 + t43325 + t43335 - t43339;
    (t51054,)
}
