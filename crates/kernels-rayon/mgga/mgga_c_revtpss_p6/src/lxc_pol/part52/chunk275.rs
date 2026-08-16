//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 275/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk275(t471: f64, t73: f64, t1248: f64, t482: f64, t1042: f64, t127: f64, t371: f64, t481: f64, t369: f64, t479: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1250 = t73 * t471;
    let t1251 = t482 * t1248 * t1250;
    let t1252 = t1042 * t1251;
    let t1256 = t371 * t127 * t482;
    let t1258 = 0.14291339372689912324e-3_f64 * t481 * t1256;
    let t1259 = t479 * t369;
    let t1260 = t475 * t1259;
    (t1250, t1251, t1252, t1256, t1258, t1260)
}
