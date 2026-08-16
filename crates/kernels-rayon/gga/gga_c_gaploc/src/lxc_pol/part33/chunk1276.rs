//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1276/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1276(t1991: f64, t33680: f64, t590: f64, t11065: f64, t5577: f64, t1029: f64, t23099: f64, t7396: f64, t10811: f64, t28837: f64, t2021: f64, t7372: f64, t8520: f64) -> (f64, f64, f64, f64, f64) {
    let t33683 = 0.2044956050875773316e1_f64 * t1991 * t33680 * t590;
    let t33685 = 0.51123901271894332902e1_f64 * t5577 * t11065;
    let t33689 = t23099 * t1029 * t7396;
    let t33690 = 0.38342925953920749676e0_f64 * t33689;
    let t33691 = t10811 * t28837;
    let t33692 = 0.17875244975925213335e0_f64 * t33691;
    let t33694 = t2021 * t8520 * t7372;
    (t33683, t33685, t33690, t33692, t33694)
}
