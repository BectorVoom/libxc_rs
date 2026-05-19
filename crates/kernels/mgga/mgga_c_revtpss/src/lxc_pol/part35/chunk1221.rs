//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1221/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1221<F: Float>(t106006: F, t106010: F, t106014: F, t106022: F, t113171: F, t113173: F, t113177: F, t113180: F, t113182: F, t113184: F, t113186: F, t113188: F, t95666: F, t98964: F) -> F {
    let t115673 = F::cast_from(0.10289764348336736873e-1_f64) * t113171 - F::cast_from(0.25724410870841842183e-2_f64) * t113173 + F::cast_from(0.12196800674228478774e-2_f64) * t106006 - F::cast_from(0.96037800584476210818e-1_f64) * t106010 - F::cast_from(0.34299214494455789578e-2_f64) * t113177 + F::cast_from(0.48018900292238105409e-1_f64) * t106014 - F::cast_from(0.51448821741683684367e-1_f64) * t113180 + F::cast_from(0.10289764348336736873e-1_f64) * t113182 + F::cast_from(0.51448821741683684367e-2_f64) * t113184 - F::cast_from(0.20579528696673473747e-1_f64) * t113186 + F::cast_from(0.10289764348336736873e-1_f64) * t113188 - F::cast_from(0.91464571985215438874e-3_f64) * t98964 + t95666 + F::cast_from(0.30492001685571196935e-2_f64) * t106022;
    t115673
}
