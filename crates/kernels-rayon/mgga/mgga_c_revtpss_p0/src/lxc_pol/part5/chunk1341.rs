//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1341/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1341(t20747: f64, t247: f64, t3719: f64, t369: f64, t6593: f64, t475: f64, t467: f64, t1260: f64, t17307: f64, t1256: f64, t6602: f64, t6595: f64) -> (f64, f64, f64, f64, f64) {
    let t21267 = t247 * t3719 * t20747;
    let t21270 = t6593 * t369;
    let t21271 = t475 * t21270;
    let t21272 = t467 * t21271;
    let t21275 = t17307 * t1260;
    let t21283 = t6602 * t1256;
    let t21285 = t6595 * t1256;
    (t21267, t21272, t21275, t21283, t21285)
}
