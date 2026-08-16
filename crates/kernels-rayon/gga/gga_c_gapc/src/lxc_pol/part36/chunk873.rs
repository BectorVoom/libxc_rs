//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 873/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk873(t3478: f64, t575: f64, t1104: f64, t1615: f64, t1112: f64, t1617: f64, t3537: f64, t687: f64, t2011: f64, t8622: f64, t8626: f64, t8629: f64, t8632: f64, t8634: f64, t8637: f64, t8641: f64, t8645: f64, t8647: f64, t8650: f64, t8657: f64, t8660: f64, t8663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10526 = t3478 * t575;
    let t10529 = t1104 * t1615;
    let t10538 = t1112 * t1617;
    let t10541 = t3537 * t687;
    let t10544 = t1112 * t2011;
    let t10560 = 0.32827263770475230566e-7_f64 * t8622 - 0.11594181388521408694e-4_f64 * t8626 - 0.55603792169291016668e-2_f64 * t8629 - 0.55603792169291016668e-2_f64 * t8632 - 0.13913017666225690434e-3_f64 * t8634 - 0.22510123728325872388e-6_f64 * t8637 - 0.46497498276882732785e-5_f64 * t8641 - 0.34752370105806885418e-3_f64 * t8645 - 0.24326659074064819793e-2_f64 * t8647 + 0.42270452978984302532e-6_f64 * t8650 + 0.12328882118870421572e-6_f64 * t8657 - 0.55603792169291016668e-2_f64 * t8660 + 0.24326659074064819792e-2_f64 * t8663;
    (t10526, t10529, t10538, t10541, t10544, t10560)
}
