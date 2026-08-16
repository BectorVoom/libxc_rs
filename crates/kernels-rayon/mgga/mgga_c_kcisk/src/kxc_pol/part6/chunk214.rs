//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 214/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk214(t891: f64, t896: f64, t898: f64, t102: f64, t142: f64, t157: f64, t67: f64, t69: f64, t857: f64, t863: f64, t881: f64, t884: f64, t889: f64, t89: f64) -> (f64, f64) {
    let t899 = t891 * t896 * t898;
    let t903 = t67 * (0.11073577833333333333e-2_f64 * t857 * t157 * t89 + 1.0_f64 * t863 * t881 - 0.18311555036753159941e-3_f64 * t884 * t69 * t142 * t102 - 0.58482233974552040708e0_f64 * t889 * t899);
    (t899, t903)
}
