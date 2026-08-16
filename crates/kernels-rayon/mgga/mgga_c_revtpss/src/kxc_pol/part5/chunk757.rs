//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 757/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk757(t1169: f64, t5142: f64, t1744: f64, t3479: f64, t1168: f64, t3358: f64, t3483: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t448: f64) -> (f64, f64, f64, f64, f64) {
    let t5143 = t5142 * t1169;
    let t5146 = t1744 * t3479;
    let t5147 = t5146 * t1168;
    let t5155 = t3483 - 0.30902777777777777778e-2_f64 * t3358 - 0.30902777777777777778e-2_f64 * t5044 - 0.61805555555555555555e-2_f64 * t5049 + 0.18541666666666666667e-1_f64 * t5054 + 0.92708333333333333333e-2_f64 * t5058;
    let t5156 = t5155 * t448;
    (t5143, t5146, t5147, t5155, t5156)
}
