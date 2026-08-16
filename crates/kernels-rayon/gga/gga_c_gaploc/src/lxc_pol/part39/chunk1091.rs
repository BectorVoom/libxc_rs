//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1091/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1091(t13756: f64, t380: f64, t42873: f64, t42874: f64, t42877: f64, t42881: f64, t42883: f64, t42885: f64, t42889: f64, t42893: f64, t42896: f64, t42899: f64) -> f64 {
    let t47054 = 0.37940008847568199465e-1_f64 * t380 * t13756;
    let t47058 = t42873 + t42874 + t47054 - t42877 + t42881 - 0.28455006635676149599e-1_f64 * t42883 - 0.11856252764865062333e-2_f64 * t42885 - 0.11856252764865062333e-2_f64 * t42889 - t42893 + t42896 + t42899;
    t47058
}
