//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 712/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk712(t2030: f64, t2074: f64, t2020: f64, t2029: f64, t2026: f64, t6: f64, t616: f64, t1948: f64, t3440: f64, t6318: f64, t6321: f64, t6324: f64, t6328: f64, t6330: f64, t6342: f64, t6356: f64, t6526: f64, t6613: f64, t6619: f64, t6621: f64, t6623: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6797 = t2030 * t2074;
    let t6799 = t2020 * t2029;
    let t6800 = t6799 * t2026;
    let t6802 = t6 * t616;
    let t6803 = t6802 * t1948;
    let t6804 = t3440 * t6803;
    let t6807 = t6318 - t6321 + t6324 - t6328 + t6330 + t6342 + t6526 - t6356 + t6613 - t6619 + t6621 - t6623;
    (t6797, t6799, t6800, t6802, t6803, t6804, t6807)
}
