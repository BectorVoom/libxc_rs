//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 733/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk733(t14902: f64, t1240: f64, t2842: f64, t4239: f64, t870: f64, t1250: f64, t8232: f64, t1882: f64, t4164: f64, t4169: f64, t12001: f64, t4159: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15118 = 2.0_f64 / 9.0_f64 * t14902;
    let t15128 = t1240 * t2842;
    let t15133 = t4239 * t870;
    let t15147 = t8232 * t1250;
    let t15168 = 4.0_f64 / 9.0_f64 * t1882 * t4164;
    let t15170 = 2.0_f64 / 9.0_f64 * t1882 * t4169;
    let t15180 = t12001 * t4159;
    (t15118, t15128, t15133, t15147, t15168, t15170, t15180)
}
