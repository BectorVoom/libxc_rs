//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1377/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1377(t33576: f64, t33578: f64, t33580: f64, t33583: f64, t33585: f64, t33588: f64, t33590: f64, t33595: f64, t33601: f64, t33606: f64, t36657: f64, t33653: f64) -> (f64, f64) {
    let t36658 = -0.10793703140429833089e-5_f64 * t33576 - 0.12141398358188788626e-5_f64 * t33578 + 0.21587406280859666178e-5_f64 * t33580 + 0.10551281119038438161e-7_f64 * t33583 + 0.21102562238076876322e-7_f64 * t33585 - 0.22509399720615334744e-6_f64 * t33588 + 0.12817159869818982005e-5_f64 * t33590 + 0.20220636637604418766e-5_f64 * t33595 - 0.15018333275585850553e-5_f64 * t33601 - 0.20220636637604418766e-4_f64 * t33606 + t36657;
    let t36659 = 0.10298285674687440379e-4_f64 * t33653;
    (t36658, t36659)
}
