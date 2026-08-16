//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1160/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1160(t1824: f64, t236: f64, t22705: f64, t550: f64, t22852: f64, t1358: f64, t7715: f64, t1831: f64, t22783: f64, t5234: f64, t6951: f64, t1811: f64, t22797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26243 = t236 * t1824;
    let t26245 = t22705 * t26243 * t550;
    let t26246 = t22852 * t26245;
    let t26251 = t7715 * t1358;
    let t26255 = t22783 * t1831;
    let t26257 = t5234 * t6951;
    let t26266 = t22797 * t1811;
    (t26245, t26246, t26251, t26255, t26257, t26266)
}
