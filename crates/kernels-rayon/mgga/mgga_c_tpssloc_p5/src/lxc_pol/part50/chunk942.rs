//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 942/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk942(t1955: f64, t4693: f64, t3174: f64, t2775: f64, t387: f64, t3961: f64, t23329: f64, t221: f64, t4509: f64, t1926: f64) -> (f64, f64, f64, f64, f64) {
    let t25419 = t1955 * t4693;
    let t25420 = t3174 * t25419;
    let t25423 = t387 * t2775;
    let t25424 = t25423 * t3961;
    let t25425 = t23329 * t25424;
    let t25428 = t221 * t4509;
    let t25429 = t1926 * t25428;
    (t25419, t25420, t25424, t25425, t25429)
}
