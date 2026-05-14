//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1360/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1360<F: Float>(t34579: F, t9724: F, t33225: F, t4648: F, t7552: F, t34560: F, t4640: F, t1636: F, t18792: F, t112761: F, t34547: F, t33196: F, t112856: F, t112860: F, t112872: F, t112881: F, t116298: F, t116336: F, t33188: F, t33222: F, t34435: F, t34548: F, t34594: F, t9728: F, t9740: F) -> (F, F, F, F) {
    let t117824 = t9724 * t34579;
    let t117829 = t33225 * t7552 * t4648;
    let t117833 = t34560 * t7552 * t4640;
    let t117837 = t33225 * t18792 * t1636;
    let t117840 = t112761 * t34547;
    let t117841 = t33196 * t117840;
    let t117847 = -0.23214722222222222222e-2 * t116298 + 0.20104166666666666667e-2 * t34594 * t33188 + 0.23148148148148148148e-2 * t112856 - 0.15476481481481481481e-2 * t116336 + 0.34722222222222222222e-2 * t34435 * t33222 + 0.11574074074074074074e-2 * t112860 - 0.10722222222222222222e-1 * t117824 * t9728 - 0.38801041666666666666e-3 * t112881 + 0.17361111111111111111e-2 * t9740 * t117829 + 0.23148148148148148148e-2 * t9740 * t117833 + 0.34722222222222222222e-2 * t9740 * t117837 + 0.44675925925925925926e-3 * t117841 + 0.13402777777777777778e-2 * t112872 * t34548 + 0.13402777777777777778e-2 * t33196 * t117837;
    (t117829, t117833, t117840, t117847)
}
