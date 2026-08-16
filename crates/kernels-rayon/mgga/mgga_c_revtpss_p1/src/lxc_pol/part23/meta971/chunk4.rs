//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3283/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3283(t47337: f64, t49087: f64, t49090: f64, t49103: f64, t49105: f64, t49122: f64, t49125: f64, t49127: f64, t74677: f64, t74682: f64, t74698: f64, t74711: f64, t74714: f64, t74717: f64, t86256: f64, t86260: f64, t86264: f64, t86274: f64) -> f64 {
    let t86276 = -0.54885603034028154483e-3_f64 * t49087 + 0.97586602194502058671e-3_f64 * t49090 + 0.27107389498472794074e-4_f64 * t49103 + 0.13553694749236397038e-5_f64 * t49105 - 0.12705000702321332056e-4_f64 * t86256 - 0.85748036236139473942e-4_f64 * t86260 - 0.85748036236139473942e-4_f64 * t86264 + 0.27107389498472794075e-4_f64 * t74677 - t49122 - t49125 + 35.0_f64 / 24.0_f64 * t74682 - 0.15246000842785598468e-2_f64 * t74698 - t49127 + 0.30492001685571196935e-4_f64 * t74711 - 0.15246000842785598467e-3_f64 * t74714 + t47337 - 35.0_f64 / 72.0_f64 * t74717 + 0.17149607247227894789e-3_f64 * t86274;
    t86276
}
