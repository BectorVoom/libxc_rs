//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1155/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1155(t1592: f64, t27177: f64, t3308: f64, t11780: f64, t2201: f64, t3324: f64, t10826: f64, t25983: f64, t39885: f64, t8243: f64, t2605: f64, t37699: f64) -> (f64, f64, f64, f64, f64) {
    let t40095 = t1592 * t3308 * t27177;
    let t40098 = t2201 * t11780 * t3324;
    let t40100 = t25983 * t10826;
    let t40102 = t39885 * t8243;
    let t40107 = t37699 * t2605;
    (t40095, t40098, t40100, t40102, t40107)
}
