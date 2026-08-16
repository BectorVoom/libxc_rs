//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1042/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1042(t3270: f64, t36987: f64, t1065: f64, t2259: f64, t11002: f64, t11060: f64, t833: f64, t1299: f64, t3370: f64, t1074: f64, t6692: f64, t1275: f64, t502: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36988 = t3270 * t36987;
    let t36994 = t1065 * t2259;
    let t36995 = t11002 * t36994;
    let t37015 = t11060 * t833;
    let t37020 = t3370 * t1299;
    let t37023 = t1074 * t6692;
    let t37028 = t502 * t1275;
    (t36988, t36995, t37015, t37020, t37023, t37028)
}
