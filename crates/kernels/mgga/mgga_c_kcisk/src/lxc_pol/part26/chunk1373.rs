//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1373/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1373<F: Float>(t109508: F, t115073: F, t115077: F, t115080: F, t115090: F, t115137: F, t118866: F, t118869: F, t118872: F, t118875: F, t118878: F, t33808: F, t33854: F, t9855: F, t115955: F, t33760: F, t36521: F) -> (F, F) {
    let t120125 = 0.38691203703703703703e-3 * t118866 + 0.11574074074074074074e-2 * t109508 - 0.11607361111111111111e-2 * t118869 + 0.77382407407407407407e-3 * t118872 - 0.25794135802469135802e-3 * t118875 + 0.23214722222222222221e-2 * t118878 + t115073 - t115077 - t115080 + 0.10416666666666666667e-1 * t33808 * t9855 + 0.10416666666666666667e-1 * t33854 * t9855 - 0.23148148148148148148e-2 * t115090 + t115137;
    let t120139 = t115955 * t36521 * t33760;
    (t120125, t120139)
}
