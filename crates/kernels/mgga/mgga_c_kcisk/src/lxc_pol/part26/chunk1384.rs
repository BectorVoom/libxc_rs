//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1384/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1384<F: Float>(t109627: F, t1163: F, t34929: F, t109633: F, t115578: F, t115589: F, t115596: F, t115606: F, t119189: F, t119203: F, t119206: F, t119214: F, t119227: F, t119231: F, t32354: F, t33778: F, t33823: F, t35012: F, t35018: F, t9544: F, t9855: F) -> (F, F) {
    let t120393 = t109627 * t34929 * t1163;
    let t120409 = -0.38691203703703703703e-2 * t119189 + 0.34722222222222222222e-2 * t32354 * t35012 - 0.13402777777777777778e-2 * t109633 * t120393 + t115578 - 0.15476481481481481481e-2 * t119203 + 0.61905925925925925925e-2 * t119206 + 0.40208333333333333334e-2 * t115589 * t9855 + 0.40208333333333333334e-2 * t115596 * t9855 + 0.40208333333333333334e-2 * t33778 * t33823 + 0.10416666666666666667e-1 * t35018 * t9544 + t115606 - 0.38691203703703703703e-3 * t119214 - 0.10446625e-1 * t119227 + 0.23214722222222222221e-2 * t119231;
    (t120393, t120409)
}
