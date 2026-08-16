//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2334/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2334(t128: f64, t121: f64, t268: f64, t8779: f64, t588: f64, t9295: f64, t2508: f64, t39494: f64, t39497: f64, t692: f64, t124: f64, t138: f64, t239: f64) -> (f64, f64, f64, f64, f64) {
    let t39503 = f64::powf(t128, -0.25e1_f64);
    let t39506 = t39503 * t121 * t8779 * t268;
    let t39508 = t9295 * t588;
    let t39510 = t2508 * t39494;
    let t39512 = t692 * t39497;
    let t39515 = t138 * t124 * t239;
    (t39506, t39508, t39510, t39512, t39515)
}
