//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1052/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1052<F: Float>(t17054: F, t17061: F, t17069: F, t5520: F, t7552: F, t11966: F, t2647: F, t5437: F, t2618: F, t5444: F, t11233: F, t11241: F, t11940: F, t17025: F, t17029: F, t17034: F, t17038: F, t17042: F, t17047: F, t17051: F, t17059: F, t17067: F, t17074: F, t1994: F, t2648: F, t5440: F, t5445: F) -> (F, F, F, F, F) {
    let t18766 = 0.23214722222222222222e-2 * t17054;
    let t18768 = 0.15476481481481481481e-2 * t17061;
    let t18771 = 0.23214722222222222222e-2 * t17069;
    let t18772 = t7552 * t5520;
    let t18775 = t2647 * t11966;
    let t18776 = t18775 * t5437;
    let t18779 = t2618 * t5444;
    let t18783 = 0.23214722222222222222e-2 * t17025 + 0.25794135802469135802e-2 * t17029 + 0.46429444444444444443e-2 * t17034 - 0.15476481481481481481e-2 * t17038 - 0.15476481481481481481e-2 * t17042 + 0.46429444444444444444e-2 * t17047 - 0.38691203703703703704e-2 * t17051 - 0.193e0 * t11940 * t2648 - 0.25794135802469135802e-3 * t11233 - t18766 + 0.34822083333333333332e-2 * t17059 + t18768 + 0.23214722222222222222e-2 * t11241 - 0.34822083333333333332e-2 * t17067 - t18771 + 0.74498e-1 * t5445 * t18772 - 0.386e0 * t1994 * t18776 + 0.74498e-1 * t18779 * t5440 - 0.23214722222222222222e-2 * t17074;
    (t18772, t18775, t18776, t18779, t18783)
}
