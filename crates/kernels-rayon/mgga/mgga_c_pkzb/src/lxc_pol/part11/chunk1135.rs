//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1135/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1135(t2104: f64, t54: f64, t9257: f64, t9259: f64, t1123: f64, t300: f64, t5633: f64, t21787: f64, t2922: f64, t9292: f64, t5974: f64, t9273: f64) -> (f64, f64, f64, f64) {
    let t25351 = t2104 * t54 * t9257 * t9259;
    let t25357 = t300 * t5633 * t1123;
    let t25434 = t2922 * t21787 * t9292;
    let t25448 = t2922 * t5974 * t9273;
    (t25351, t25357, t25434, t25448)
}
