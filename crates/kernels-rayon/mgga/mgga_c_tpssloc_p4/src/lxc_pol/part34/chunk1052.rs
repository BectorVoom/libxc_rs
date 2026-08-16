//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1052/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1052(t28321: f64, t6646: f64, t1888: f64, t5544: f64, t6638: f64, t6637: f64, t6552: f64, t1894: f64, t5631: f64, t214: f64, t1880: f64, t1510: f64, t25249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28322 = t6646 * t28321;
    let t28323 = t1888 * t28322;
    let t28329 = t6638 * t5544;
    let t28330 = t6637 * t28329;
    let t28331 = t6552 * t28330;
    let t28333 = t1894 * t5631;
    let t28334 = t214 * t28333;
    let t28335 = t1880 * t28334;
    let t28337 = t25249 * t1510;
    (t28322, t28323, t28329, t28330, t28331, t28333, t28334, t28335, t28337)
}
