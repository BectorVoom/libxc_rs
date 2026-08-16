//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2701/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2701(t1907: f64, t47672: f64, t1343: f64, t1868: f64, t198: f64, t40079: f64, t4139: f64, t47152: f64, t47638: f64, t48328: f64, t48329: f64, t48330: f64, t48332: f64, t48334: f64, t48336: f64, t48421: f64, t5541: f64, t9590: f64) -> f64 {
    let t49668 = t1907 * t47672;
    let t49675 = 3.0_f64 * t1343 * t198 * t48421 + 6.0_f64 * t1868 * t4139 * t47638 - 6.0_f64 * t49668 * t5541 * t9590 - t40079 + t47152 - t48328 - t48329 + t48330 - t48332 + t48334 + t48336;
    t49675
}
