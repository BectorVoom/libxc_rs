//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1309/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1309(t31246: f64, t7756: f64, t5107: f64, t8320: f64, t1774: f64, t30991: f64, t1799: f64, t6995: f64, t22574: f64, t8643: f64, t31048: f64, t7685: f64) -> (f64, f64, f64, f64, f64) {
    let t119845 = t31246 * t7756;
    let t119850 = 2.0_f64 * t8320 * t5107;
    let t119852 = 2.0_f64 * t30991 * t1774;
    let t119853 = t1799 * t6995;
    let t119856 = 6.0_f64 * t22574 * t8643 * t119853;
    let t119858 = 3.0_f64 * t7685 * t31048;
    (t119845, t119850, t119852, t119856, t119858)
}
