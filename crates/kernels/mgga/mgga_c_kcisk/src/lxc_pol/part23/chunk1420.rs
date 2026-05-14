//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1420/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1420<F: Float>(t33802: F, t9532: F, t109697: F, t109699: F, t109701: F, t114023: F, t114051: F, t114054: F, t114057: F, t114072: F, t114092: F, t114098: F, t32354: F, t32385: F, t33832: F, t33851: F, t33864: F, t9519: F, t9529: F, t9851: F) -> (F,) {
    let t115433 = t33802 * t9532;
    let t115452 = -0.16975308641975308642e-1 * t109697 + 0.92592592592592592593e-2 * t115433 + 0.46429444444444444443e-2 * t114023 - 0.20833333333333333334e-1 * t32354 * t33832 - 0.15476481481481481481e-2 * t109699 - 0.51588271604938271604e-3 * t109701 + 0.15476481481481481481e-2 * t114051 + 0.77382407407407407407e-3 * t114054 + 0.30952962962962962962e-2 * t114057 + 0.52083333333333333333e-2 * t9851 * t32385 - 0.11607361111111111111e-1 * t114072 - 0.27777777777777777778e-1 * t33851 * t9519 - 0.27777777777777777778e-1 * t9529 * t33864 - 0.41270617283950617284e-2 * t114092 + 0.38691203703703703704e-2 * t114098;
    (t115452,)
}
