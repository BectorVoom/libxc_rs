//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1172/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1172<F: Float>(t14616: F, t5176: F, t14849: F, t14578: F, t3438: F, t3437: F, t14801: F, t14804: F, t14807: F, t14810: F, t14813: F, t14817: F, t14819: F, t14821: F, t14823: F, t14825: F, t14827: F, t14830: F, t14834: F, t14836: F, t14840: F, t14843: F, t14845: F, t14847: F) -> (F, F, F) {
    let t14850 = t5176 * t14616;
    let t14851 = t14849 * t14850;
    let t14853 = t3438 * t14578;
    let t14854 = t3437 * t14853;
    let t14856 = t14801 / F::cast_from(256.0_f64) + F::cast_from(19.0_f64) / F::cast_from(144.0_f64) * t14804 + t14807 / F::cast_from(12.0_f64) - t14810 / F::cast_from(24.0_f64) - t14813 / F::cast_from(128.0_f64) - t14817 / F::cast_from(16.0_f64) + t14819 / F::cast_from(3.0_f64) - t14821 / F::cast_from(192.0_f64) + t14823 / F::cast_from(96.0_f64) + t14825 / F::cast_from(24.0_f64) - t14827 / F::cast_from(576.0_f64) - t14830 / F::cast_from(128.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t14834 - t14836 / F::cast_from(18.0_f64) + t14840 / F::cast_from(54.0_f64) - t14843 / F::cast_from(576.0_f64) + t14845 / F::cast_from(256.0_f64) - t14847 / F::cast_from(12.0_f64) + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t14851 + t14854 / F::cast_from(192.0_f64);
    (t14851, t14854, t14856)
}
