//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 790/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk790<F: Float>(t1251: F, t3499: F, t3514: F, t5300: F, t5322: F, t6759: F, t6763: F, t6767: F, t6771: F, t6776: F, t6839: F, t1268: F, t1240: F, t1857: F, t430: F, t5003: F, t5345: F, t6558: F, t6561: F, t6564: F, t6616: F, t6622: F, t6627: F, t6631: F, t6738: F, t6835: F) -> (F, F, F) {
    let t6842 = -t3499 + t5300 / 864.0 - t5322 / 288.0 + t1251 * t6759 / 432.0 - t3514 * t6763 / 288.0 - t1251 * t6767 / 288.0 + t1251 * t6771 / 576.0 + t1251 * t6776 / 96.0 - t1251 * t6839 / 192.0;
    let t6843 = t6842 * t1268;
    let t6855 = 0.11607361111111111111e-2 * t6558 - 0.34822083333333333332e-2 * t6561 + 0.23214722222222222222e-2 * t6564 - 0.17411041666666666666e-2 * t6616 - 0.66725e-1 * t1240 * t6843 - 0.13345e0 * t5345 * t1857 + 0.15476481481481481481e-2 * t5003 + 0.66725e-1 * t1240 * t6738 + t6835 * t430 - 0.23214722222222222222e-2 * t6622 + 0.15476481481481481481e-2 * t6627 - 0.23214722222222222222e-2 * t6631;
    (t6842, t6843, t6855)
}
