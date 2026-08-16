//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 678/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk678(t4907: f64, t617: f64, t608: f64, t163: f64, t1774: f64, t24: f64, t5005: f64, t10933: f64, t3118: f64, t353: f64, t579: f64, t609: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10982 = 1.0_f64 / t4907 / t617;
    let t10983 = t608 * t10982;
    let t10999 = t163 * t1774;
    let t11003 = t24 * t5005;
    let t11030 = 0.93011851851851851854e0_f64 * t10933;
    let t11032 = t353 * t3118 * t579;
    let t11033 = 0.73028148148148148147e0_f64 * t11032;
    let t11036 = 1.0_f64 / t609 / t615 / 8.0_f64;
    (t10983, t10999, t11003, t11030, t11032, t11033, t11036)
}
