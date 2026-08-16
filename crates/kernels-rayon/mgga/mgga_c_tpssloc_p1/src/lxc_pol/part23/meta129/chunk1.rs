//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 635/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk635(t2349: f64, t5480: f64, t5396: f64, t103: f64, t100: f64, t104: f64, t1447: f64, t1450: f64, t5469: f64, t5472: f64, t5475: f64, t92: f64) -> (f64, f64, f64, f64) {
    let t5481 = t2349 * t5480;
    let t5484 = -t5396;
    let t5485 = t103 * t5484;
    let t5488 = 10.0_f64 / 9.0_f64 * t92 * t5469 + 5.0_f64 / 3.0_f64 * t92 * t5472 + 40.0_f64 / 9.0_f64 * t5475 * t104 - 50.0_f64 / 9.0_f64 * t1447 * t1450 + 10.0_f64 / 9.0_f64 * t100 * t5481 + 5.0_f64 / 3.0_f64 * t100 * t5485;
    (t5481, t5484, t5485, t5488)
}
