//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3283/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3283<F: Float>(t47337: F, t49087: F, t49090: F, t49103: F, t49105: F, t49122: F, t49125: F, t49127: F, t74677: F, t74682: F, t74698: F, t74711: F, t74714: F, t74717: F, t86256: F, t86260: F, t86264: F, t86274: F) -> F {
    let t86276 = -F::cast_from(0.54885603034028154483e-3_f64) * t49087 + F::cast_from(0.97586602194502058671e-3_f64) * t49090 + F::cast_from(0.27107389498472794074e-4_f64) * t49103 + F::cast_from(0.13553694749236397038e-5_f64) * t49105 - F::cast_from(0.12705000702321332056e-4_f64) * t86256 - F::cast_from(0.85748036236139473942e-4_f64) * t86260 - F::cast_from(0.85748036236139473942e-4_f64) * t86264 + F::cast_from(0.27107389498472794075e-4_f64) * t74677 - t49122 - t49125 + F::new(35.0) / F::new(24.0) * t74682 - F::cast_from(0.15246000842785598468e-2_f64) * t74698 - t49127 + F::cast_from(0.30492001685571196935e-4_f64) * t74711 - F::cast_from(0.15246000842785598467e-3_f64) * t74714 + t47337 - F::new(35.0) / F::new(72.0) * t74717 + F::cast_from(0.17149607247227894789e-3_f64) * t86274;
    t86276
}
