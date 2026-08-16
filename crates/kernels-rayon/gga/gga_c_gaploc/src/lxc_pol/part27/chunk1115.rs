//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1115/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1115(t2672: f64, t6134: f64, t7372: f64, t23176: f64, t9820: f64, t10024: f64, t23348: f64, t787: f64, t5533: f64, t883: f64, t900: f64, t10023: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29014 = 0.59584149919750711116e-1_f64 * t6134 * t2672 * t7372;
    let t29016 = 0.11916829983950142223e0_f64 * t9820 * t23176;
    let t29019 = 0.17875244975925213335e0_f64 * t787 * t23348 * t10024;
    let t29020 = t883 * t5533;
    let t29021 = t900 * t29020;
    let t29023 = 0.20854452471912748891e0_f64 * t10023 * t29021;
    (t29014, t29016, t29019, t29020, t29021, t29023)
}
