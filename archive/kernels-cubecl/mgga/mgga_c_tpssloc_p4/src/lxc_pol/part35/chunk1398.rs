//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1398/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1398<F: Float>(t1845: F, t6330: F, t24995: F, t8643: F, t28239: F, t7685: F, t22633: F, t22635: F, t26337: F, t6460: F, t1985: F, t7700: F, t97511: F) -> (F, F, F, F) {
    let t106971 = t6330 * t1845;
    let t106974 = F::cast_from(18.0_f64) * t24995 * t8643 * t106971;
    let t106978 = F::cast_from(3.0_f64) * t7685 * t28239;
    let t106982 = t22633 * t22635 * t26337 * t6460;
    let t106986 = t1985 * t97511 * t7700;
    (t106974, t106978, t106982, t106986)
}
