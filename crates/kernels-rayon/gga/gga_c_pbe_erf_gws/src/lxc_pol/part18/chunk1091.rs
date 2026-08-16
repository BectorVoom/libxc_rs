//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1091/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1091(t3912: f64, t4473: f64, t833: f64, t2391: f64, t3916: f64, t3721: f64, t938: f64, t2409: f64, t9296: f64, t11889: f64, t831: f64, t1143: f64, t2416: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12198 = t3912 * t4473;
    let t12199 = t12198 * t833;
    let t12201 = t3916 * t2391;
    let t12204 = t3721 * t938;
    let t12206 = t2409 * t9296 * t12204;
    let t12210 = t2409 * t831 * t11889;
    let t12213 = t1143 * t2416;
    (t12198, t12199, t12201, t12204, t12206, t12210, t12213)
}
