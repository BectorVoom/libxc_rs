//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 758/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk758(t1748: f64, t360: f64, t1181: f64, t1532: f64, t372: f64, t1165: f64, t1552: f64, t407: f64, t495: f64, t1753: f64, t322: f64, t1163: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5710 = t1748 * t360;
    let t5712 = t1181 * t1532 * t5710;
    let t5715 = t1748 * t372;
    let t5717 = t1165 * t1552 * t5715;
    let t5720 = t407 * t495;
    let t5722 = t1165 * t1532 * t5720;
    let t5725 = t1753 * t322;
    let t5726 = t1532 * t5725;
    let t5727 = t1181 * t5726;
    let t5728 = t1163 * t5727;
    (t5710, t5712, t5715, t5717, t5720, t5722, t5725, t5726, t5727, t5728)
}
