//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3491/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3491<F: Float>(t11710: F, t19730: F, t3091: F, t11672: F, t11875: F, t15604: F, t15716: F, t19572: F, t19645: F, t19731: F, t247: F, t3116: F, t3117: F, t42176: F, t53407: F, t53413: F, t53416: F, t53422: F, t53427: F, t53431: F, t53433: F, t65071: F) -> F {
    let t65738 = t3091 * t11710 * t19730;
    let t65753 = -F::cast_from(0.3811023832717309953e-3_f64) * t53407 - F::cast_from(0.28582678745379824648e-3_f64) * t53413 + F::cast_from(0.57165357490759649296e-3_f64) * t53416 - F::cast_from(0.95275595817932748826e-4_f64) * t42176 - F::cast_from(0.3811023832717309953e-3_f64) * t53422 - F::cast_from(0.30488190661738479624e-2_f64) * t11672 * t19731 + F::cast_from(0.3811023832717309953e-3_f64) * t65738 - F::cast_from(0.12862205435420921092e-2_f64) * t15716 * t247 * t3116 * t65071 + F::cast_from(0.96545937095505185476e-2_f64) * t53427 - F::cast_from(0.3811023832717309953e-3_f64) * t53431 + F::cast_from(0.3811023832717309953e-3_f64) * t53433 - F::cast_from(0.15244095330869239812e-2_f64) * t11672 * t19645 + F::cast_from(0.42874018118069736972e-3_f64) * t11875 * t3117 * t19572 * t15604;
    t65753
}
