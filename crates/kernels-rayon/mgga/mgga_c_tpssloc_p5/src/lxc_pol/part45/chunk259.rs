//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 259/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk259(t1102: f64, t1107: f64, t281: f64, t415: f64, t904: f64, t241: f64, t457: f64, t1090: f64, t136: f64, t1092: f64, t1103: f64, t1105: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1108 = t1107 * t1102;
    let t1111 = t281 * t904 * t415;
    let t1112 = 0.82156666666666666667e-1_f64 * t1111;
    let t1113 = t241 * t457;
    let t1114 = t1113 * t1090;
    let t1115 = t136 * t1114;
    let t1117 = 0.1898925e1_f64 * t1103 - t1105 + 0.29896666666666666667e0_f64 * t1092 + 0.3071625e0_f64 * t1108 - t1112 + 0.82156666666666666667e-1_f64 * t1115;
    (t1108, t1111, t1113, t1114, t1115, t1117)
}
