//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1012/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1012(t6080: f64, t10263: f64, t10264: f64, t10266: f64, t10267: f64, t10268: f64, t10269: f64, t4826: f64, t4837: f64, t4840: f64, t4843: f64, t4846: f64, t4849: f64, t4856: f64, t4864: f64, t8031: f64, t8033: f64, t8034: f64, t8035: f64) -> f64 {
    let t11313 = 0.6846054806677777778e0_f64 * t6080;
    let t11314 = t10263 - t10264 - t10266 + t4826 + t10267 + t10268 - t8031 - t4837 - t4840 - t4843 + t4846 + t4849 + t10269 + t8033 + t11313 + t4856 + t8034 - t8035 - t4864;
    t11314
}
