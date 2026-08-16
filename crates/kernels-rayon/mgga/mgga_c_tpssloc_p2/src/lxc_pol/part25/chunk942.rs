//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 942/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk942(t12132: f64, t17: f64, t3826: f64, t592: f64, t1285: f64, t2225: f64, t2371: f64, t3691: f64, t1294: f64, t9494: f64, t2535: f64, t12121: f64, t12123: f64, t12125: f64, t12128: f64, t12131: f64, t9853: f64, t9859: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12133 = t17 * t12132;
    let t12134 = t592 * t3826;
    let t12135 = 24.0_f64 * t12134;
    let t12136 = t2225 * t1285;
    let t12137 = 60.0_f64 * t12136;
    let t12138 = t3691 * t2371;
    let t12139 = 0.35089341735807877242e1_f64 * t12138;
    let t12141 = 0.10254018858216406658e4_f64 * t1294 * t9494;
    let t12142 = t3691 * t2535;
    let t12143 = 0.17544670867903938621e1_f64 * t12142;
    let t12144 = t12121 + t12123 + t12125 + t12128 + t12131 + t12133 - t12135 + t12137 + t9853 + t12139 + t9859 - t12141 - t12143;
    (t12133, t12135, t12137, t12139, t12141, t12143, t12144)
}
