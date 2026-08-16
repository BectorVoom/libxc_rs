//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1489/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1489(t109: f64, t79816: f64, t5493: f64, t5449: f64, t5456: f64, t53777: f64, t53779: f64, t56099: f64, t56104: f64, t73967: f64, t53798: f64, t1799: f64, t19596: f64, t20067: f64, t20675: f64, t28830: f64, t3918: f64, t39249: f64, t39256: f64, t39261: f64, t5160: f64, t5161: f64, t6347: f64, t74068: f64, t75240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t79817 = piecewise3(t110, 0.0_f64, t79816);
    let t79825 = t5493 * t5493;
    let t79829 = t5449 * t5456;
    let t79834 = 0.86748650402413918736e-1_f64 * t53777;
    let t79835 = 0.1301229756036208781e0_f64 * t53779;
    let t79836 = 0.10389515463408878255e3_f64 * t56099;
    let t79837 = 0.35089341735807877242e1_f64 * t56104;
    let t79853 = 0.73245789224026180216e-3_f64 * t73967;
    let t79854 = 0.14035736694323150897e2_f64 * t53798;
    let t79855 = 12.0_f64 * t1799 * t3918 * t74068 + 24.0_f64 * t1799 * t3918 * t75240 - 36.0_f64 * t19596 * t28830 * t3918 + 18.0_f64 * t20067 * t3918 * t6347 - 4.0_f64 * t20675 * t5160 * t5161 - t39249 - t39256 - t39261 - t79834 - t79835 - t79836 - t79837 - t79853 - t79854;
    (t79817, t79825, t79829, t79834, t79835, t79836, t79837, t79853, t79854, t79855)
}
