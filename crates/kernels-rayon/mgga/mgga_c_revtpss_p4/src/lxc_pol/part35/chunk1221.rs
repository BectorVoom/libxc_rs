//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1221/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1221(t106006: f64, t106010: f64, t106014: f64, t106022: f64, t113171: f64, t113173: f64, t113177: f64, t113180: f64, t113182: f64, t113184: f64, t113186: f64, t113188: f64, t95666: f64, t98964: f64) -> f64 {
    let t115673 = 0.10289764348336736873e-1_f64 * t113171 - 0.25724410870841842183e-2_f64 * t113173 + 0.12196800674228478774e-2_f64 * t106006 - 0.96037800584476210818e-1_f64 * t106010 - 0.34299214494455789578e-2_f64 * t113177 + 0.48018900292238105409e-1_f64 * t106014 - 0.51448821741683684367e-1_f64 * t113180 + 0.10289764348336736873e-1_f64 * t113182 + 0.51448821741683684367e-2_f64 * t113184 - 0.20579528696673473747e-1_f64 * t113186 + 0.10289764348336736873e-1_f64 * t113188 - 0.91464571985215438874e-3_f64 * t98964 + t95666 + 0.30492001685571196935e-2_f64 * t106022;
    t115673
}
