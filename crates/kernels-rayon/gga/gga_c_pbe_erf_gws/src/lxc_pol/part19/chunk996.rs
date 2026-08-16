//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 996/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk996(t10738: f64, t10739: f64, t10741: f64, t10745: f64, t10749: f64, t10751: f64, t10837: f64, t10838: f64, t10840: f64, t5359: f64, t7578: f64, t7617: f64, t7619: f64, t7623: f64, t7665: f64, t7668: f64) -> f64 {
    let t11216 = t10738 + t7578 - t10739 - t10741 - t10745 + t10749 + t10751 - t10837 + t7617 + t7619 + t7623 - t10838 + t5359 - t10840 - t7665 - t7668;
    t11216
}
