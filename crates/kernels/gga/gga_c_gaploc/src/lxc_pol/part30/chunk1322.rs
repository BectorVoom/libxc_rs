//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1322/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1322<F: Float>(t32435: F, t739: F, t1991: F, t590: F, t11065: F, t5577: F, t1029: F, t23099: F, t7396: F, t10811: F, t28837: F, t2021: F, t7372: F, t8520: F) -> (F, F, F, F, F) {
    let t33680 = t739 * t32435;
    let t33683 = F::cast_from(0.2044956050875773316e1_f64) * t1991 * t33680 * t590;
    let t33685 = F::cast_from(0.51123901271894332902e1_f64) * t5577 * t11065;
    let t33689 = t23099 * t1029 * t7396;
    let t33690 = F::cast_from(0.38342925953920749676e0_f64) * t33689;
    let t33691 = t10811 * t28837;
    let t33692 = F::cast_from(0.17875244975925213335e0_f64) * t33691;
    let t33694 = t2021 * t8520 * t7372;
    (t33683, t33685, t33690, t33692, t33694)
}
