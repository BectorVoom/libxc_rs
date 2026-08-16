//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1003/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1003(t1137: f64, t4777: f64, t4781: f64, t4597: f64, t4496: f64, t1140: f64, t4590: f64, t4480: f64, t3431: f64, t4963: f64, t3409: f64, t4991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16824 = t1137 * t4777;
    let t16826 = t1137 * t4781;
    let t16839 = t1137 * t4597;
    let t16841 = t1137 * t4496;
    let t16847 = t1140 * t4590;
    let t16849 = t1140 * t4480;
    let t16863 = t3431 * t4963;
    let t16865 = t3409 * t4991;
    (t16824, t16826, t16839, t16841, t16847, t16849, t16863, t16865)
}
