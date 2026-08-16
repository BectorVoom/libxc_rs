//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 997/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk997(t10845: f64, t10847: f64, t10850: f64, t10852: f64, t10856: f64, t10859: f64, t10863: f64, t10866: f64, t10870: f64, t10873: f64, t10875: f64, t5945: f64, t5948: f64, t5952: f64, t5954: f64, t7672: f64, t7715: f64) -> f64 {
    let t11219 = t7672 + 8.0_f64 / 3.0_f64 * t5945 + t5948 + t5952 + t10845 - t10847 + t10850 + t10852 - t10856 - t10859 + t10863 + t10866 - t7715 + 0.11181742741110338156e-1_f64 * t5954 - t10870 - t10873 - t10875;
    t11219
}
