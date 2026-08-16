//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3197/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3197(t1174: f64, t18225: f64, t3431: f64, t18221: f64, t15522: f64, t4889: f64, t11668: f64, t11678: f64, t1177: f64, t15686: f64, t3248: f64, t3252: f64, t3440: f64, t3494: f64, t3577: f64, t3578: f64, t52893: f64, t53270: f64, t53272: f64, t53274: f64, t53287: f64, t53291: f64, t5979: f64, t6225: f64, t63368: f64, t63410: f64, t64990: f64) -> f64 {
    let t66449 = t1174 * t3431 * t18225;
    let t66452 = t1174 * t3431 * t18221;
    let t66458 = t4889 * t15522;
    let t66480 = t53270 / 324.0_f64 - t53272 / 3456.0_f64 - t53274 / 972.0_f64 - t53287 / 1728.0_f64 - t66449 / 108.0_f64 - t66452 / 72.0_f64 - t1174 * t1177 * t63410 / 72.0_f64 - t53291 / 1728.0_f64 - 4.0_f64 / 243.0_f64 * t66458 - 4.0_f64 / 27.0_f64 * t4889 * t15686 + t1174 * t3440 * t63368 / 36.0_f64 - t3577 * t3578 * t5979 * t3494 / 4608.0_f64 + 5.0_f64 / 576.0_f64 * t52893 * t11668 * t64990 - t11678 * t3578 * t6225 * t3252 / 2304.0_f64 - t11678 * t3578 * t6225 * t3248 / 1152.0_f64;
    t66480
}
