//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1398/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1398(t4227: f64, t8589: f64, t829: f64, t830: f64, t54619: f64, t54621: f64, t55889: f64, t55901: f64, t57639: f64, t57641: f64, t57643: f64, t57648: f64, t57652: f64, t57654: f64, t57657: f64, t57661: f64, t57663: f64, t57666: f64, t827: f64) -> f64 {
    let t58896 = t8589 * t4227;
    let t58898 = t829 * t830 * t58896;
    let t58902 = t55889 - t54619 - 35.0_f64 / 108.0_f64 * t54621 - t57639 / 48.0_f64 + 7.0_f64 / 576.0_f64 * t57641 + 7.0_f64 / 144.0_f64 * t57643 - t57648 / 384.0_f64 + 7.0_f64 / 1152.0_f64 * t57652 + t57654 / 12.0_f64 + t57657 / 24.0_f64 + t57661 / 24.0_f64 + t57663 / 48.0_f64 - t827 * t58898 / 48.0_f64 - t57666 / 48.0_f64 + t55901;
    t58902
}
