//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1172/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1172(t14616: f64, t5176: f64, t14849: f64, t14578: f64, t3438: f64, t3437: f64, t14801: f64, t14804: f64, t14807: f64, t14810: f64, t14813: f64, t14817: f64, t14819: f64, t14821: f64, t14823: f64, t14825: f64, t14827: f64, t14830: f64, t14834: f64, t14836: f64, t14840: f64, t14843: f64, t14845: f64, t14847: f64) -> (f64, f64, f64) {
    let t14850 = t5176 * t14616;
    let t14851 = t14849 * t14850;
    let t14853 = t3438 * t14578;
    let t14854 = t3437 * t14853;
    let t14856 = t14801 / 256.0_f64 + 19.0_f64 / 144.0_f64 * t14804 + t14807 / 12.0_f64 - t14810 / 24.0_f64 - t14813 / 128.0_f64 - t14817 / 16.0_f64 + t14819 / 3.0_f64 - t14821 / 192.0_f64 + t14823 / 96.0_f64 + t14825 / 24.0_f64 - t14827 / 576.0_f64 - t14830 / 128.0_f64 - 3.0_f64 / 8.0_f64 * t14834 - t14836 / 18.0_f64 + t14840 / 54.0_f64 - t14843 / 576.0_f64 + t14845 / 256.0_f64 - t14847 / 12.0_f64 + 3.0_f64 / 128.0_f64 * t14851 + t14854 / 192.0_f64;
    (t14851, t14854, t14856)
}
