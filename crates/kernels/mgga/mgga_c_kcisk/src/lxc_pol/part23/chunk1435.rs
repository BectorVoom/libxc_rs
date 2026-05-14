//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1435/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1435<F: Float>(t114633: F, t114635: F, t115710: F, t3936: F, t32466: F, t5675: F, t14609: F, t21499: F, t533: F, t109626: F, t109633: F, t110029: F, t110037: F, t114643: F, t115263: F, t115267: F, t115669: F, t115713: F, t115796: F, t32339: F, t32354: F, t32439: F, t33771: F, t33906: F, t33911: F, t33916: F) -> (F, F) {
    let t115831 = 0.15476481481481481481e-2 * t114633;
    let t115846 = 0.23214722222222222222e-2 * t114635;
    let t115849 = t3936 * t115710;
    let t115851 = t115849 * t5675 * t32466;
    let t115858 = t14609 * t533 * t21499;
    let t115861 = t115831 + 0.46296296296296296296e-2 * t109626 * t115713 - 0.92592592592592592592e-2 * t32339 * t33906 - 0.92592592592592592592e-2 * t32339 * t33911 - 0.18518518518518518518e-1 * t32339 * t33916 + 0.6701388888888888889e-3 * t32439 * t115263 + 0.89351851851851851853e-3 * t32439 * t115267 + 0.34722222222222222222e-2 * t32354 * t33771 + t115846 - 0.34722222222222222222e-2 * t109626 * t115796 - 0.26805555555555555556e-2 * t109633 * t115851 - 0.15476481481481481481e-2 * t114643 - 0.25794135802469135802e-3 * t110029 + 0.23214722222222222222e-2 * t110037 - 0.77602083333333333334e-3 * t115858 * t115669;
    (t115851, t115861)
}
