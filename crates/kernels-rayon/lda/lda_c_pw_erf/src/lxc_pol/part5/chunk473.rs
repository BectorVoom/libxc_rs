//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 473/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk473(t159: f64, t1904: f64, t285: f64, t477: f64, t780: f64, t281: f64, t684: f64, t872: f64, t1096: f64, t1158: f64, t1161: f64, t1165: f64, t1176: f64, t1181: f64, t1189: f64, t1195: f64, t1740: f64, t2237: f64, t2303: f64, t279: f64, t296: f64) -> (f64, f64, f64, f64, f64) {
    let t2306 = t1904 * t159 * t285;
    let t2310 = t780 * t477 * t285;
    let t2311 = t281 * t2310;
    let t2313 = t684 * t872;
    let t2315 = -0.01197423401025461_f64 * t1176 - t1181 - t1189 - t1740 + t1158 - 0.0002905674151788692_f64 * t1161 - t1165 + t1195 - 0.054045904796391424_f64 * t1096 + t2237 * t296 + t2303 * t279 - 0.01197423401025461_f64 * t281 * t2306 - 0.01197423401025461_f64 * t2311 + 0.019957056683757683_f64 * t2313;
    (t2306, t2310, t2311, t2313, t2315)
}
