//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1221/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1221<F: Float>(t106006: F, t106010: F, t106014: F, t106022: F, t113171: F, t113173: F, t113177: F, t113180: F, t113182: F, t113184: F, t113186: F, t113188: F, t95666: F, t98964: F) -> F {
    let t115673 = F::new(0.10289764348336736873e-1) * t113171 - F::new(0.25724410870841842183e-2) * t113173 + F::new(0.12196800674228478774e-2) * t106006 - F::new(0.96037800584476210818e-1) * t106010 - F::new(0.34299214494455789578e-2) * t113177 + F::new(0.48018900292238105409e-1) * t106014 - F::new(0.51448821741683684367e-1) * t113180 + F::new(0.10289764348336736873e-1) * t113182 + F::new(0.51448821741683684367e-2) * t113184 - F::new(0.20579528696673473747e-1) * t113186 + F::new(0.10289764348336736873e-1) * t113188 - F::new(0.91464571985215438874e-3) * t98964 + t95666 + F::new(0.30492001685571196935e-2) * t106022;
    t115673
}
