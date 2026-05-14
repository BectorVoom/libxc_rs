//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1206/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1206<F: Float>(t1589: F, t34988: F, t1586: F, t7736: F, t9537: F, t3952: F, t7744: F, t1312: F, t7740: F, t2737: F, t33501: F, t33530: F, t33535: F, t33564: F, t34759: F, t34931: F, t34969: F, t9536: F, t9855: F, t9860: F, t9869: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34989 = t1589 * t34988;
    let t34990 = t1586 * t34989;
    let t35003 = t9537 * t7736;
    let t35004 = t3952 * t35003;
    let t35007 = t9537 * t7744;
    let t35008 = t1312 * t35007;
    let t35011 = t9537 * t7740;
    let t35012 = t1312 * t35011;
    let t35016 = 0.52083333333333333333e-2 * t2737 * t34969 + 0.52083333333333333333e-2 * t2737 * t34990 + 0.10416666666666666667e-1 * t9860 * t9855 + 0.15476481481481481481e-2 * t33501 + 0.15476481481481481481e-2 * t33530 - 0.23214722222222222222e-2 * t33535 + 0.10416666666666666667e-1 * t9860 * t9869 - 0.23214722222222222222e-2 * t34759 - 0.10416666666666666667e-1 * t9536 * t34931 - 0.23148148148148148148e-2 * t9536 * t35004 - 0.17361111111111111111e-2 * t9536 * t35008 + 0.34722222222222222222e-2 * t9536 * t35012 + 0.23214722222222222222e-2 * t33564;
    (t34989, t34990, t35003, t35004, t35007, t35008, t35011, t35012, t35016)
}
