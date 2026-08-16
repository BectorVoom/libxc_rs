//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 766/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk766(t1211: f64, t5230: f64, t1294: f64, t1774: f64, t1277: f64, t3358: f64, t3579: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64) -> (f64, f64, f64) {
    let t5231 = t1211 * t5230;
    let t5236 = t1774 * t1294;
    let t5237 = t1277 * t5236;
    let t5245 = t3579 - 0.4938888888888888889e-2_f64 * t3358 - 0.4938888888888888889e-2_f64 * t5044 - 0.9877777777777777778e-2_f64 * t5049 + 0.29633333333333333334e-1_f64 * t5054 + 0.14816666666666666667e-1_f64 * t5058;
    (t5231, t5237, t5245)
}
