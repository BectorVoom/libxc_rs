//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1046/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1046(t5886: f64, t8078: f64, t1411: f64, t2152: f64, t8072: f64, t1450: f64, t3785: f64, t2231: f64, t7831: f64, t1415: f64, t30494: f64, t3776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31152 = t5886 * t8078;
    let t31153 = t1411 * t31152;
    let t31165 = t8072 * t2152;
    let t31166 = t1450 * t31165;
    let t31167 = t3785 * t31166;
    let t31168 = t1411 * t31167;
    let t31170 = t7831 * t2231;
    let t31171 = t1450 * t31170;
    let t31172 = t1415 * t31171;
    let t31173 = t1411 * t31172;
    let t31175 = t3776 * t30494;
    (t31153, t31165, t31168, t31170, t31173, t31175)
}
