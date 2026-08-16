//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 973/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk973(t322: f64, t1079: f64, t1305: f64, t1081: f64, t1312: f64, t1310: f64, t3386: f64, t839: f64, t11059: f64) -> (f64, f64, f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t11087 = t1079 * t1305;
    let t11092 = t1312 * t1081;
    let t11106 = t1310 * t1081;
    let t11108 = t839 * t3386;
    let t11110 = piecewise3(t332, 0.0_f64, t11059);
    (t11087, t11092, t11106, t11108, t11110)
}
