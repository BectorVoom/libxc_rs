//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1174/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1174(t10868: f64, t7614: f64, t7615: f64, t11714: f64, t6493: f64, t1592: f64, t27177: f64, t3308: f64, t11780: f64, t2201: f64, t3324: f64, t10826: f64, t25983: f64) -> (f64, f64, f64, f64, f64) {
    let t40090 = t7614 * t10868 * t7615;
    let t40092 = t6493 * t11714;
    let t40095 = t1592 * t3308 * t27177;
    let t40098 = t2201 * t11780 * t3324;
    let t40100 = t25983 * t10826;
    (t40090, t40092, t40095, t40098, t40100)
}
