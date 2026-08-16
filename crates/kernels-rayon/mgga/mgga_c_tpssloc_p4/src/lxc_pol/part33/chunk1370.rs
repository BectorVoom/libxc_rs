//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1370/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1370(t106758: f64, t22544: f64, t26013: f64, t26016: f64, t27937: f64, t27950: f64, t27953: f64, t7428: f64, t7442: f64, t7446: f64, t90137: f64, t96426: f64, t96443: f64, t96454: f64, t96462: f64, t96470: f64, t96473: f64) -> f64 {
    let t106780 = -15.0_f64 * t22544 * t106758 + 30.0_f64 * t90137 * t96426 - 10.0_f64 * t96443 * t26013 - 5.0_f64 * t96473 * t26013 - 10.0_f64 * t26016 * t96454 - 10.0_f64 * t26016 * t96462 - 5.0_f64 * t26016 * t96470 - t27937 * t7442 / 2.0_f64 - t27937 * t7446 / 2.0_f64 - t7428 * t27950 / 2.0_f64 - t7428 * t27953;
    t106780
}
