//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2071/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2071<F: Float>(t14648: F, t775: F, t14832: F, t2661: F, t2652: F, t4345: F, t10716: F, t4349: F, t10746: F, t10749: F, t10756: F, t10758: F, t14817: F, t14820: F, t14823: F, t14825: F, t14829: F, t2730: F) -> (F, F, F, F, F, F) {
    let t14833 = t14648 * t775;
    let t14834 = t14832 * t14833;
    let t14836 = F::cast_from(0.28582678745379824648e-3_f64) * t2661 * t14834;
    let t14837 = t2652 * t4345;
    let t14839 = t10716 * t4349;
    let t14841 = F::cast_from(0.50820002809285328224e-5_f64) * t10746 - F::cast_from(0.36143185997963725432e-4_f64) * t10749 - F::cast_from(0.18071592998981862717e-4_f64) * t14817 + F::cast_from(0.25410001404642664112e-5_f64) * t14820 - t14823 + t2730 * t14825 / F::cast_from(8.0_f64) + t2730 * t14829 / F::cast_from(16.0_f64) - t14836 + F::cast_from(0.80031500487063509014e-2_f64) * t14837 + F::cast_from(0.54208002996571016773e-3_f64) * t14839 - t10756 - t10758;
    (t14833, t14834, t14836, t14837, t14839, t14841)
}
