//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1110/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1110(t14667: f64, t22045: f64, t2365: f64, t549: f64, t7069: f64, t7390: f64, t5638: f64, t822: f64, t9419: f64, t20671: f64, t22624: f64, t28831: f64, t825: f64, t969: f64) -> (f64, f64, f64, f64, f64) {
    let t28851 = 0.59584149919750711116e-1_f64 * t14667 * t2365 * t22045;
    let t28854 = 0.11916829983950142223e0_f64 * t7390 * t549 * t7069;
    let t28856 = t822 * t5638 * t9419;
    let t28859 = 0.51123901271894332902e0_f64 * t28856 * t20671 * t22624;
    let t28861 = t825 * t969 * t28831;
    (t28851, t28854, t28856, t28859, t28861)
}
