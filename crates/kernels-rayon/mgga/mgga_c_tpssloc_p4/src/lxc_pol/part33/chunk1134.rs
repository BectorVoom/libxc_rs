//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1134/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1134(t23384: f64, t7557: f64, t3216: f64, t7627: f64, t28: f64, t870: f64, t10143: f64, t1437: f64, t1864: f64, t1863: f64, t1410: f64, t2240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25824 = t23384 * t7557;
    let t25840 = t7627 * t3216;
    let t25891 = t870 * t28;
    let t25927 = t10143 * t28;
    let t26012 = t1864 * t1437;
    let t26013 = t1863 * t26012;
    let t26016 = t2240 * t1410;
    (t25824, t25840, t25891, t25927, t26012, t26013, t26016)
}
