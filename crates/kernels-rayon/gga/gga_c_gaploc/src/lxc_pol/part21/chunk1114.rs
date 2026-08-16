//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1114/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1114(t22166: f64, t22333: f64, t23021: f64, t787: f64, t9824: f64, t10024: f64, t1980: f64, t7442: f64, t2586: f64, t4752: f64, t10007: f64, t1710: f64, t825: f64, t9438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29025 = 0.59584149919750711116e-1_f64 * t22166 * t22333;
    let t29030 = t787 * t23021;
    let t29032 = 0.29792074959875355558e-1_f64 * t29030 * t9824;
    let t29035 = 0.17875244975925213335e0_f64 * t1980 * t7442 * t10024;
    let t29052 = t4752 * t2586;
    let t29074 = t825 * t9438 * t10007 * t1710;
    (t29025, t29030, t29032, t29035, t29052, t29074)
}
