//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 736/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk736(t1900: f64, t227: f64, t5519: f64, t1937: f64, t690: f64, t1936: f64, t244: f64, t239: f64) -> (f64, f64, f64, f64, f64) {
    let t5804 = 1.0_f64 / t1900 / t227;
    let t5812 = 0.53272592592592592592e-1_f64 * t5519;
    let t5825 = t690 * t1937;
    let t5829 = 1.0_f64 / t1936 / t244;
    let t5830 = t239 * t5829;
    (t5804, t5812, t5825, t5829, t5830)
}
