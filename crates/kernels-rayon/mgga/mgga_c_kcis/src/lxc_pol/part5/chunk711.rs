//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 711/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk711(t421: f64, t993: f64, t4581: f64, t737: f64, t992: f64, t1253: f64, t167: f64, t1852: f64, t25: f64, t1251: f64, t1851: f64, t330: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5310 = t993 * t421;
    let t5311 = t5310 * t4581;
    let t5314 = t737 * t992;
    let t5315 = t1253 * t167;
    let t5316 = t5314 * t5315;
    let t5321 = t25 * t1852;
    let t5322 = t1251 * t5321;
    let t5324 = t1851 * t330;
    (t5310, t5311, t5315, t5316, t5321, t5322, t5324)
}
