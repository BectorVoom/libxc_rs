//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 684/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk684(t1550: f64, t537: f64, t113: f64, t2115: f64, t1604: f64, t1553: f64, t1567: f64, t1569: f64, t1616: f64, t560: f64, t2201: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5167 = t537 * t1550;
    let t5168 = t5167 * t113;
    let t5169 = t2115 * t5168;
    let t5170 = t1604 * t5169;
    let t5172 = t1567 * t1553;
    let t5173 = t5172 * t1569;
    let t5174 = t2115 * t5173;
    let t5175 = t1604 * t5174;
    let t5177 = t1616 * t560;
    let t5179 = t2201 * t785 * t5177;
    (t5168, t5169, t5170, t5173, t5174, t5175, t5179)
}
