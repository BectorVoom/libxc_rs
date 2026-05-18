//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1172/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1172<F: Float>(t14616: F, t5176: F, t14849: F, t14578: F, t3438: F, t3437: F, t14801: F, t14804: F, t14807: F, t14810: F, t14813: F, t14817: F, t14819: F, t14821: F, t14823: F, t14825: F, t14827: F, t14830: F, t14834: F, t14836: F, t14840: F, t14843: F, t14845: F, t14847: F) -> (F, F, F) {
    let t14850 = t5176 * t14616;
    let t14851 = t14849 * t14850;
    let t14853 = t3438 * t14578;
    let t14854 = t3437 * t14853;
    let t14856 = t14801 / F::new(256.0) + F::new(19.0) / F::new(144.0) * t14804 + t14807 / F::new(12.0) - t14810 / F::new(24.0) - t14813 / F::new(128.0) - t14817 / F::new(16.0) + t14819 / F::new(3.0) - t14821 / F::new(192.0) + t14823 / F::new(96.0) + t14825 / F::new(24.0) - t14827 / F::new(576.0) - t14830 / F::new(128.0) - F::new(3.0) / F::new(8.0) * t14834 - t14836 / F::new(18.0) + t14840 / F::new(54.0) - t14843 / F::new(576.0) + t14845 / F::new(256.0) - t14847 / F::new(12.0) + F::new(3.0) / F::new(128.0) * t14851 + t14854 / F::new(192.0);
    (t14851, t14854, t14856)
}
