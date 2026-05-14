//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1205/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1205<F: Float>(t33767: F, t33846: F, t33871: F, t33874: F, t33884: F, t34725: F, t34728: F, t34737: F, t34740: F, t34945: F, t34969: F, t9516: F, t9851: F, t9855: F, t9869: F, t1597: F, t8398: F) -> (F, F) {
    let t34984 = -0.60312500000000000001e-2 * t9516 * t34945 + 0.10416666666666666667e-1 * t9851 * t9869 + 0.20104166666666666667e-2 * t9516 * t34969 - 0.34722222222222222222e-2 * t33846 - 0.23214722222222222222e-2 * t34725 + 0.23214722222222222222e-2 * t34728 + 0.34722222222222222222e-2 * t33871 + 0.13402777777777777778e-2 * t33874 + 0.40208333333333333334e-2 * t33767 * t9855 + 0.10416666666666666667e-1 * t9851 * t9855 - 0.11574074074074074074e-2 * t33884 + 0.17411041666666666666e-2 * t34737 + 0.15476481481481481481e-2 * t34740;
    let t34988 = t1597 * t8398;
    (t34984, t34988)
}
