//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1095/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1095(t1: f64, t128: f64, t2580: f64, t33598: f64, t350: f64, t126: f64, t15541: f64, t190: f64, t1903: f64, t314: f64, t442: f64, t7953: f64) -> (f64, f64) {
    let t33606 = t33598 * t2580 * t128 * t1 * t350;
    let t33614 = t7953 * t126 * t1903 * t15541 * t314 * t190 * t442;
    (t33606, t33614)
}
