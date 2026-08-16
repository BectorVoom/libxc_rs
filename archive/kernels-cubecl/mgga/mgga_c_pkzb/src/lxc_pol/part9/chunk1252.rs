//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1252/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1252<F: Float>(t2009: F, t759: F, t2099: F, t757: F, t7577: F, t2908: F, t5945: F, t1125: F, t17955: F, t2096: F, t7581: F, t18126: F, t18142: F, t18145: F, t18150: F, t18158: F, t18167: F, t2104: F, t21686: F, t300: F, t5695: F, t5729: F, t5956: F, t7736: F, t7742: F, t7796: F) -> (F, F) {
    let t21912 = t2009 * t759;
    let t21928 = t757 * t2099 * t7577;
    let t21930 = t5945 * t2908;
    let t21933 = t757 * t17955 * t1125;
    let t21935 = t2096 * t7581;
    let t21937 = -F::cast_from(0.15434646522505105311e-1_f64) * t2104 * t300 * t7796 * t5695 + F::cast_from(0.38586616306262763276e-2_f64) * t7736 * t21686 * t5956 * t21912 - F::cast_from(0.38586616306262763275e-2_f64) * t7742 * t21686 * t5729 * t21912 + F::cast_from(0.25724410870841842183e-2_f64) * t18126 + F::cast_from(0.14481890564325777822e-1_f64) * t18142 - F::cast_from(0.85748036236139473944e-3_f64) * t18145 + F::cast_from(0.14291339372689912324e-3_f64) * t18150 + F::cast_from(0.85748036236139473944e-3_f64) * t18158 + F::cast_from(0.14291339372689912324e-3_f64) * t18167 + F::cast_from(0.42874018118069736972e-3_f64) * t21928 + F::cast_from(0.14481890564325777821e-1_f64) * t21930 + F::cast_from(0.63517063878621832551e-4_f64) * t21933 + F::cast_from(0.7622047665434619906e-3_f64) * t21935;
    (t21912, t21937)
}
