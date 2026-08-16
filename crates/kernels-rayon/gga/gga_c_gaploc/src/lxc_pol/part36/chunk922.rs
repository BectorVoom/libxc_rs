//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 922/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk922(t42898: f64, t42863: f64, t42866: f64, t42867: f64, t42868: f64, t42869: f64, t42870: f64, t42871: f64, t42872: f64, t42873: f64, t42874: f64, t42877: f64, t42881: f64, t42883: f64, t42885: f64, t42889: f64, t42893: f64, t42896: f64) -> f64 {
    let t42899 = 0.23712505529730124666e-2_f64 * t42898;
    let t42900 = t42863 + t42866 - t42867 + t42868 - t42869 - t42870 - t42871 + t42872 + t42873 + t42874 - t42877 + t42881 - 0.56910013271352299198e-1_f64 * t42883 - 0.23712505529730124666e-2_f64 * t42885 - 0.23712505529730124666e-2_f64 * t42889 - t42893 + t42896 + t42899;
    t42900
}
