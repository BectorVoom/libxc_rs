//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 676/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk676(t489: f64, t57: f64, t1620: f64, t2215: f64, t543: f64, t108: f64, t110: f64, t548: f64, t122: f64, t2161: f64, t625: f64, t505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5119 = t57 * t489;
    let t5123 = t1620 * t2215;
    let t5132 = t543 * t543;
    let t5134 = t108 / t5132;
    let t5135 = t5134 * t110;
    let t5136 = t5135 * t548;
    let t5146 = t2161 * t122;
    let t5147 = t625 * t5146;
    let t5148 = t505 * t108;
    (t5119, t5123, t5134, t5136, t5147, t5148)
}
