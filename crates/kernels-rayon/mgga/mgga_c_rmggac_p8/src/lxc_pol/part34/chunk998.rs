//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 998/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk998(t77470: f64, t2010: f64, t2415: f64, t8188: f64, t14434: f64, t5898: f64, t75092: f64, t75100: f64, t75103: f64, t75106: f64, t75108: f64, t75110: f64, t75115: f64, t77450: f64, t77452: f64, t77458: f64, t77463: f64, t77464: f64, t77465: f64, t77468: f64, t884: f64) -> f64 {
    let t77471 = 0.36021158228745895953e-3_f64 * t77470;
    let t77473 = t2010 * t2415 * t8188;
    let t77474 = 0.36021158228745895953e-3_f64 * t77473;
    let t77475 = -t77450 - 0.8759653046450683594e-6_f64 * t75092 + t77452 - 0.58171619854173713846e-5_f64 * t75100 - 0.72714524817717142308e-5_f64 * t75103 - 0.10511583655740820313e-5_f64 * t75106 - 0.58171619854173713846e-5_f64 * t75108 - t77458 + t75110 + t75115 - 0.11974241701863808564e0_f64 * t884 * t14434 * t5898 + t77463 - t77464 + t77465 - t77468 - t77471 - t77474;
    t77475
}
