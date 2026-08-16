//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 920/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk920(t509: f64, t526: f64, t235: f64, t72: f64, t1242: f64, t2376: f64, t339: f64, t1250: f64, t1184: f64, t3211: f64, t498: f64, t7622: f64) -> (f64, f64, f64, f64, f64) {
    let t9984 = 1.0_f64 / t526 / t509;
    let t9986 = t235 * t9984 * t72;
    let t9994 = t339 * t1242 * t2376;
    let t9995 = t9994 * t1250;
    let t10016 = t3211 * t1184;
    let t10019 = 24.0_f64 * t7622 * t498;
    (t9986, t9994, t9995, t10016, t10019)
}
