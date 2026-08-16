//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 666/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk666(t5: f64, t1497: f64, t2242: f64, t2247: f64, t4171: f64, t4173: f64, t4178: f64, t4241: f64, t603: f64, t644: f64, t91: f64, t117: f64, t116: f64, t1501: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t4245 = piecewise3(t8, 0.0_f64, -4.0_f64 * t1497 * t2242 + 20.0_f64 * t2247 * t4178 + t4171 * t91 - 4.0_f64 * t4173 * t644 - 4.0_f64 * t4241 * t603);
    let t4246 = t4245 * t117;
    let t4248 = t1501 * t116;
    (t4245, t4246, t4248)
}
