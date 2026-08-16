//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 608/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk608(t642: f64, t695: f64, t1060: f64, t1757: f64, t5192: f64, t5182: f64, t1801: f64, t4644: f64, t1800: f64, t1799: f64, t1755: f64, t654: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5193 = t642 * t695;
    let t5194 = t1060 * t1757;
    let t5195 = t5193 * t5194;
    let t5196 = t5192 * t5195;
    let t5197 = t5182 * t5196;
    let t5199 = t1801 * t4644;
    let t5200 = t1800 * t5199;
    let t5201 = t1799 * t5200;
    let t5203 = t654 * t1755;
    (t5193, t5196, t5197, t5199, t5200, t5201, t5203)
}
