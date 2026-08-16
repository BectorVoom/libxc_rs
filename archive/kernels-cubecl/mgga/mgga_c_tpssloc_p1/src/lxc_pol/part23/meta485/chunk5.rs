//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1489/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1489<F: Float>(t109: F, t79816: F, t5493: F, t5449: F, t5456: F, t53777: F, t53779: F, t56099: F, t56104: F, t73967: F, t53798: F, t1799: F, t19596: F, t20067: F, t20675: F, t28830: F, t3918: F, t39249: F, t39256: F, t39261: F, t5160: F, t5161: F, t6347: F, t74068: F, t75240: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t79817 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t79816);
    let t79825 = t5493 * t5493;
    let t79829 = t5449 * t5456;
    let t79834 = F::cast_from(0.86748650402413918736e-1_f64) * t53777;
    let t79835 = F::cast_from(0.1301229756036208781e0_f64) * t53779;
    let t79836 = F::cast_from(0.10389515463408878255e3_f64) * t56099;
    let t79837 = F::cast_from(0.35089341735807877242e1_f64) * t56104;
    let t79853 = F::cast_from(0.73245789224026180216e-3_f64) * t73967;
    let t79854 = F::cast_from(0.14035736694323150897e2_f64) * t53798;
    let t79855 = F::cast_from(12.0_f64) * t1799 * t3918 * t74068 + F::cast_from(24.0_f64) * t1799 * t3918 * t75240 - F::cast_from(36.0_f64) * t19596 * t28830 * t3918 + F::cast_from(18.0_f64) * t20067 * t3918 * t6347 - F::cast_from(4.0_f64) * t20675 * t5160 * t5161 - t39249 - t39256 - t39261 - t79834 - t79835 - t79836 - t79837 - t79853 - t79854;
    (t79817, t79825, t79829, t79834, t79835, t79836, t79837, t79853, t79854, t79855)
}
