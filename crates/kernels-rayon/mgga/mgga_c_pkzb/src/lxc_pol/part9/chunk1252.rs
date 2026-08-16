//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1252/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1252(t2009: f64, t759: f64, t2099: f64, t757: f64, t7577: f64, t2908: f64, t5945: f64, t1125: f64, t17955: f64, t2096: f64, t7581: f64, t18126: f64, t18142: f64, t18145: f64, t18150: f64, t18158: f64, t18167: f64, t2104: f64, t21686: f64, t300: f64, t5695: f64, t5729: f64, t5956: f64, t7736: f64, t7742: f64, t7796: f64) -> (f64, f64) {
    let t21912 = t2009 * t759;
    let t21928 = t757 * t2099 * t7577;
    let t21930 = t5945 * t2908;
    let t21933 = t757 * t17955 * t1125;
    let t21935 = t2096 * t7581;
    let t21937 = -0.15434646522505105311e-1_f64 * t2104 * t300 * t7796 * t5695 + 0.38586616306262763276e-2_f64 * t7736 * t21686 * t5956 * t21912 - 0.38586616306262763275e-2_f64 * t7742 * t21686 * t5729 * t21912 + 0.25724410870841842183e-2_f64 * t18126 + 0.14481890564325777822e-1_f64 * t18142 - 0.85748036236139473944e-3_f64 * t18145 + 0.14291339372689912324e-3_f64 * t18150 + 0.85748036236139473944e-3_f64 * t18158 + 0.14291339372689912324e-3_f64 * t18167 + 0.42874018118069736972e-3_f64 * t21928 + 0.14481890564325777821e-1_f64 * t21930 + 0.63517063878621832551e-4_f64 * t21933 + 0.7622047665434619906e-3_f64 * t21935;
    (t21912, t21937)
}
