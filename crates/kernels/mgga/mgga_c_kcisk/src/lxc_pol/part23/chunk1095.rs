//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1095/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1095<F: Float>(t19966: F, t14612: F, t2331: F, t4348: F, t20002: F, t4350: F, t6587: F, t1596: F, t14181: F, t14195: F, t14218: F, t14220: F, t14224: F, t14609: F, t14636: F, t1557: F, t19985: F, t19988: F, t19994: F, t19996: F, t20000: F, t20007: F, t20010: F, t21469: F, t2332: F, t4324: F, t4514: F, t6426: F, t6592: F) -> (F, F, F, F, F) {
    let t21988 = 0.61905925925925925925e-2 * t19966;
    let t21992 = t2331 * t14612;
    let t21993 = t21992 * t4348;
    let t22002 = 0.15476481481481481481e-2 * t20002;
    let t22009 = t6587 * t4350;
    let t22010 = t22009 * t1596;
    let t22016 = -0.11607361111111111111e-2 * t14181 + 0.15476481481481481481e-2 * t14195 - t21988 + 0.77382407407407407407e-3 * t14218 - 0.23214722222222222222e-2 * t14220 + 0.17024129629629629629e-1 * t19985 - 0.43134342e-1 * t14609 * t21993 - 0.193e0 * t6426 * t4514 - 0.34822083333333333332e-2 * t19988 + 0.15476481481481481481e-2 * t19994 - 0.23214722222222222222e-2 * t19996 - 0.23214722222222222222e-2 * t20000 - t22002 - 0.11607361111111111111e-2 * t20007 - 0.193e0 * t14636 * t2332 - 0.77382407407407407406e-3 * t20010 + 0.193e0 * t1557 * t21469 + 0.386e0 * t1557 * t22010 + 0.386e0 * t4324 * t6592 + 0.77382407407407407406e-3 * t14224;
    (t21992, t21993, t22009, t22010, t22016)
}
