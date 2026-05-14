//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1229/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1229<F: Float>(t103662: F, t27339: F, t102655: F, t102658: F, t102661: F, t102666: F, t102669: F, t103069: F, t28388: F, t98830: F, t98845: F, t98849: F, t98854: F, t98864: F, t102674: F, t102678: F, t102681: F, t102684: F, t102687: F, t102694: F, t102698: F, t102701: F, t2239: F, t23157: F, t28403: F, t29404: F, t3964: F, t7916: F, t8151: F, t98874: F) -> (F, F) {
    let t103702 = t27339 * t103662;
    let t103712 = 0.88437037037037037033e-2 * t102655 - 0.61890573922526041667e-5 * t103702 + 0.13265555555555555555e-1 * t102658 - 0.88437037037037037033e-2 * t102661 - 0.7369753086419753086e-3 * t98830 - 0.37134344353515625e-4 * t28388 * t103069 + 0.1621345679012345679e-1 * t102666 - 0.92673611111111111112e-3 * t98845 - t98849 - t98854 + 0.16581944444444444444e-2 * t102669 - t98864;
    let t103731 = 0.16581944444444444444e-2 * t102674 - 0.67960648148148148147e-2 * t3964 * t23157 * t2239 - 0.37069444444444444444e-2 * t8151 * t28403 + 0.67960648148148148147e-2 * t29404 * t7916 + 0.13265555555555555555e-1 * t102678 - 0.82376543209876543213e-3 * t98874 - 0.55273148148148148147e-3 * t102681 + 0.11054629629629629629e-2 * t102684 - 0.33163888888888888888e-2 * t102687 + 0.11054629629629629629e-2 * t102694 + 0.22109259259259259258e-2 * t102698 - 0.44218518518518518516e-2 * t102701;
    (t103712, t103731)
}
