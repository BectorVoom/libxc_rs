//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 950/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk950(t1065: f64, t792: f64, t11002: f64, t1102: f64, t3314: f64, t3457: f64, t2304: f64, t875: f64, t3434: f64, t3439: f64, t1266: f64, t321: f64) -> (f64, f64, f64, f64, f64) {
    let t11003 = t1065 * t792;
    let t11004 = t11002 * t11003;
    let t11008 = t1102 * t3314 * t3457;
    let t11015 = t2304 * t875;
    let t11017 = t3434 * t11015 * t3439;
    let t11018 = 0.1951603679568577289e-3_f64 * t11017;
    let t11031 = t1266 * t321;
    (t11004, t11008, t11015, t11018, t11031)
}
