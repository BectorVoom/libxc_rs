//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 934/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk934<F: Float>(t17061: F, t17069: F, t11966: F, t2647: F, t2618: F, t5444: F, t17076: F, t17086: F, t5439: F, t7644: F, t17739: F, t17750: F, t17757: F, t17765: F, t2656: F, t5531: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t18768 = 0.15476481481481481481e-2 * t17061;
    let t18771 = 0.23214722222222222222e-2 * t17069;
    let t18775 = t2647 * t11966;
    let t18779 = t2618 * t5444;
    let t18785 = 0.15476481481481481481e-2 * t17076;
    let t18787 = 0.10317654320987654321e-2 * t17086;
    let t18792 = t7644 * t5439;
    let t18826 = 0.10317654320987654321e-2 * t17739;
    let t18829 = 0.15476481481481481481e-2 * t17750;
    let t18831 = 0.30952962962962962962e-2 * t17757;
    let t18833 = 0.25794135802469135802e-2 * t17765;
    let t18925 = t2656 * t5531;
    (t18768, t18771, t18775, t18779, t18785, t18787, t18792, t18826, t18829, t18831, t18833, t18925)
}
