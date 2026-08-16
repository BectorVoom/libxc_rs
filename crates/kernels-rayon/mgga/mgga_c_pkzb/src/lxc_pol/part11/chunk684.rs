//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 684/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk684(t25: f64, t26: f64, t1447: f64, t40: f64, t31: f64, t1499: f64, t466: f64, t1531: f64, t1508: f64, t49: f64, t512: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4809 = t26 * t25;
    let t4810 = 1.0_f64 / t4809;
    let t4827 = 1.0_f64 / t1447 / t40;
    let t4828 = t31 * t4827;
    let t4865 = t466 * t1499;
    let t4867 = 0.16265371950452609763e-1_f64 * t1531 * t4865;
    let t4868 = t466 * t1508;
    let t4870 = 0.48159733137676571078e0_f64 * t1531 * t4868;
    let t4871 = t512 * t49;
    (t4810, t4827, t4828, t4865, t4867, t4868, t4870, t4871)
}
