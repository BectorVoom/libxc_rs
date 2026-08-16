//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 1003/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk1003(t15199: f64, t15252: f64, t15307: f64, t15341: f64, t15401: f64, t15453: f64, t15496: f64, t15543: f64, t14911: f64, t14914: f64, t15074: f64, t15126: f64, t15129: f64, t15131: f64, t15134: f64, t15136: f64, t15138: f64, t15140: f64, t301: f64, t317: f64) -> f64 {
    let t15546 = t15199 + t15252 + t15307 + t15341 + t15401 + t15453 + t15496 + t15543;
    let t15548 = -2.0_f64 * t14911 * t317 - t14914 * t317 - t15546 * t301 - 2.0_f64 * t15074 + 2.0_f64 * t15126 + 4.0_f64 * t15129 + 8.0_f64 * t15131 - 4.0_f64 * t15134 - 2.0_f64 * t15136 - 2.0_f64 * t15138 - 4.0_f64 * t15140;
    t15548
}
