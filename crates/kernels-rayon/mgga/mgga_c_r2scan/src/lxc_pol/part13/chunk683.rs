//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 683/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk683(t5125: f64, t514: f64, t2252: f64, t788: f64, t2201: f64, t785: f64, t543: f64, t108: f64, t110: f64, t548: f64, t1632: f64, t2185: f64) -> (f64, f64, f64, f64, f64) {
    let t5126 = t514 * t5125;
    let t5128 = t788 * t2252;
    let t5130 = t2201 * t785 * t5128;
    let t5132 = t543 * t543;
    let t5134 = t108 / t5132;
    let t5135 = t5134 * t110;
    let t5136 = t5135 * t548;
    let t5142 = t1632 * t2185;
    (t5126, t5130, t5134, t5136, t5142)
}
