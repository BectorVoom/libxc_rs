//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1003/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1003(t1212: f64, t30551: f64, t3722: f64, t19580: f64, t7754: f64, t2092: f64, t7753: f64, t1191: f64, t3677: f64, t2093: f64, t7785: f64, t3639: f64) -> (f64, f64, f64, f64, f64) {
    let t30553 = t3722 * t30551 * t1212;
    let t30557 = 6.0_f64 * t19580 * t7754;
    let t30558 = t7753 * t2092;
    let t30559 = t30558 * t1191;
    let t30561 = 6.0_f64 * t3677 * t30559;
    let t30562 = t2093 * t7785;
    let t30564 = 6.0_f64 * t3639 * t30562;
    (t30553, t30557, t30558, t30561, t30564)
}
