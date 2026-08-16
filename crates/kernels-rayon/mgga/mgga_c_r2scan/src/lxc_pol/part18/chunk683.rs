//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 683/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk683(t5251: f64, t597: f64, t5250: f64, t1823: f64, t732: f64, t1818: f64, t712: f64, t1822: f64, t234: f64, t716: f64, t224: f64, t719: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5252 = t597 * t5251;
    let t5253 = t5250 * t5252;
    let t5258 = t732 * t1823;
    let t5260 = t1818 * t712;
    let t5261 = t5260 * t1822;
    let t5263 = 0.30762056574649219973e4_f64 * t234 * t5261;
    let t5265 = t716 * t716;
    let t5266 = 1.0_f64 / t5265;
    let t5267 = t5266 * t224;
    let t5268 = t719 * t719;
    (t5252, t5253, t5258, t5263, t5266, t5267, t5268)
}
