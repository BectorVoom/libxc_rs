//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1030/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1030<F: Float>(t23748: F, t655: F, t2455: F, t7202: F, t10866: F, t17218: F, t17220: F, t17222: F, t17317: F, t23413: F, t23416: F, t23421: F, t5013: F, t664: F, t7235: F, t7239: F, t7243: F, sigma2: F) -> (F,) {
    let t23749 = t23748 * sigma2;
    let t23750 = t23749 * t655;
    let t23753 = t7202 * t2455;
    let t23761 = -0.95950873152945691804e-1 * t17218 + 0.63967248768630461203e-1 * t17220 - 0.23987718288236422951e-1 * t17222 + 0.17990788716177317213e-1 * t23413 - 0.11993859144118211475e-1 * t23416 + 0.47975436576472845901e-1 * t17317 * t7235 - 0.17990788716177317213e-1 * t5013 * t23421 + 0.5397236614853195164e-1 * t23750 * t664 - 0.28785261945883707542e0 * t23753 * t664 - 0.35981577432354634426e-1 * t17317 * t7239 - 0.71963154864709268852e-1 * t17317 * t7243 + 0.11993859144118211475e-1 * t10866;
    (t23761,)
}
