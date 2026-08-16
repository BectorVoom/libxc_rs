//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 947/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk947(t17396: f64, t17421: f64, t1056: f64, t1037: f64, t1459: f64, t5170: f64, t8688: f64, t8686: f64, t1460: f64, t14852: f64, t4144: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17422 = t17396 + t17421;
    let t17423 = t17422 * t1056;
    let t17425 = 1.0_f64 * t1037 * t17423;
    let t17426 = t5170 * t1459;
    let t17427 = t17426 * t8688;
    let t17429 = 0.51725014705706168417e3_f64 * t8686 * t17427;
    let t17431 = 3.0_f64 * t14852 * t1460;
    let t17433 = 3.0_f64 * t4144 * t5187;
    (t17422, t17423, t17425, t17426, t17427, t17429, t17431, t17433)
}
