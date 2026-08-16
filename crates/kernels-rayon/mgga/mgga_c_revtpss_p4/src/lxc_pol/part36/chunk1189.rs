//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1189/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1189(t1936: f64, t30138: f64, t4248: f64, t7741: f64, t5920: f64, t93: f64, t7889: f64, t1312: f64, t30004: f64, t18245: f64, t1937: f64, t7735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30140 = 4.0_f64 * t30138 * t1936;
    let t30142 = 4.0_f64 * t4248 * t7741;
    let t30143 = t93 * t5920;
    let t30145 = 2.0_f64 * t30143 * t1936;
    let t30147 = 4.0_f64 * t7889 * t7741;
    let t30149 = 2.0_f64 * t1312 * t30004;
    let t30154 = 2.0_f64 * t18245 * t1937;
    let t30156 = 4.0_f64 * t30138 * t1937;
    let t30158 = 4.0_f64 * t4248 * t7735;
    (t30140, t30142, t30143, t30145, t30147, t30149, t30154, t30156, t30158)
}
