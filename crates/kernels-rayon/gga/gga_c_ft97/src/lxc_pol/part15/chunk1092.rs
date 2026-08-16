//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1092/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1092(t12680: f64, t144: f64, t1901: f64, t20874: f64, t2222: f64, t3439: f64, t40766: f64, t4431: f64, t446: f64, t4668: f64, t4828: f64, t51149: f64, t63258: f64, t78584: f64, t78601: f64, t78603: f64, t78605: f64, t78618: f64, t87220: f64, t87462: f64, t9133: f64) -> f64 {
    let t87805 = -112.0_f64 / 81.0_f64 * t51149 - 4.0_f64 / 3.0_f64 * t1901 * t9133 * t2222 * t4431 * t4668 + 8.0_f64 / 9.0_f64 * t78584 - 8.0_f64 / 9.0_f64 * t1901 * t3439 * t40766 * t87462 - 4.0_f64 / 3.0_f64 * t446 * t144 * t87220 + 4.0_f64 / 27.0_f64 * t78601 - 8.0_f64 / 27.0_f64 * t78603 + 8.0_f64 / 3.0_f64 * t78605 - 8.0_f64 / 3.0_f64 * t1901 * t12680 * t20874 + 4.0_f64 / 3.0_f64 * t1901 * t63258 * t4828 - 8.0_f64 / 3.0_f64 * t78618;
    t87805
}
