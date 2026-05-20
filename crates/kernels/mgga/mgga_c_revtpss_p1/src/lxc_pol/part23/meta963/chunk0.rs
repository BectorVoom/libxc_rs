//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3255/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3255<F: Float>(t6869: F, t73856: F, t9816: F, t9818: F, t1353: F, t22079: F, t22096: F, t22876: F, t3934: F, t3936: F, t46730: F, t48489: F, t73994: F, t73998: F, t74010: F, t74015: F, t74017: F, t74022: F, t74024: F, t74029: F, t74033: F, t74037: F, t74174: F, t74184: F, t74186: F, t74206: F, t800: F) -> F {
    let t85735 = t9816 * t9818 * t73856 * t6869;
    let t85738 = -t48489 + F::cast_from(0.25724410870841842183e-2_f64) * t73994 - F::cast_from(0.85748036236139473944e-3_f64) * t73998 - F::cast_from(0.15246000842785598468e-3_f64) * t74010 - F::cast_from(0.42874018118069736972e-3_f64) * t74015 + F::cast_from(0.45732285992607719437e-3_f64) * t74017 + F::cast_from(0.21437009059034868486e-4_f64) * t74022 + F::new(5.0) / F::new(4.0) * t46730 * t800 * t22876 * t1353 + F::cast_from(0.18292914397043087774e-2_f64) * t74024 - F::cast_from(0.85748036236139473944e-4_f64) * t74029 + F::cast_from(0.42874018118069736972e-4_f64) * t74033 + F::cast_from(0.21437009059034868486e-4_f64) * t74037 - F::cast_from(0.38115002106963996168e-4_f64) * t74174 + F::cast_from(0.15246000842785598468e-3_f64) * t74184 - F::cast_from(0.60023625365297631762e-2_f64) * t74186 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t22079 * t22096 + F::cast_from(0.15246000842785598467e-3_f64) * t85735 + F::cast_from(0.30492001685571196935e-3_f64) * t74206;
    t85738
}
