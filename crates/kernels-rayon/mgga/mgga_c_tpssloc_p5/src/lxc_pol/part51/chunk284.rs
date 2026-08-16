//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 284/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk284(t1111: f64, t241: f64, t457: f64, t1090: f64, t136: f64, t1092: f64, t1103: f64, t1105: f64, t1108: f64, t422: f64, t1099: f64, t1086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1112 = 0.82156666666666666667e-1_f64 * t1111;
    let t1113 = t241 * t457;
    let t1114 = t1113 * t1090;
    let t1115 = t136 * t1114;
    let t1117 = 0.1898925e1_f64 * t1103 - t1105 + 0.29896666666666666667e0_f64 * t1092 + 0.3071625e0_f64 * t1108 - t1112 + 0.82156666666666666667e-1_f64 * t1115;
    let t1118 = 1.0_f64 / t422;
    let t1119 = t1117 * t1118;
    let t1121 = 1.0_f64 * t1099 * t1119;
    let t1122 = 0.17123333333333333333e-1_f64 * t1086;
    (t1112, t1113, t1114, t1115, t1117, t1118, t1119, t1121, t1122)
}
