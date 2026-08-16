//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1353/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1353(t19577: f64, t22574: f64, t36533: f64, t8449: f64, t8944: f64, t26164: f64, t3701: f64, t5187: f64, t1983: f64, t31084: f64, t26504: f64, t8450: f64) -> (f64, f64, f64, f64) {
    let t120663 = 6.0_f64 * t22574 * t36533 * t19577;
    let t120664 = t8449 * t8944;
    let t120665 = t120664 * t26164;
    let t120669 = t3701 * t5187;
    let t120672 = 3.0_f64 * t1983 * t31084 * t120669;
    let t120675 = t8450 * t26504;
    (t120663, t120665, t120672, t120675)
}
