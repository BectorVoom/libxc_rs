//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1041/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1041<F: Float>(t27508: F, t27681: F, t19735: F, t19757: F, t21402: F, t26514: F, t26518: F, t26753: F, t26755: F, t26758: F, t26762: F, t26764: F, t26767: F, t26770: F, t26776: F, t26780: F, t26783: F, t26785: F, t26787: F, t26792: F, t548: F) -> (F, F) {
    let t27682 = t27508 + t27681;
    let t27685 = 0.19345601851851851852e-2 * t26514 - 0.15476481481481481481e-2 * t26518 - 0.17411041666666666666e-2 * t26753 - 0.11607361111111111111e-2 * t26755 - 0.23214722222222222222e-2 * t26758 + 0.46429444444444444444e-2 * t26762 - 0.15476481481481481481e-2 * t26764 + 0.69644166666666666664e-2 * t26767 - 0.23214722222222222221e-2 * t26770 + 0.30952962962962962962e-2 * t19735 + t21402 + 0.77382407407407407407e-3 * t26776 + 0.23214722222222222221e-2 * t26780 + 0.92858888888888888885e-2 * t26783 + 0.15476481481481481481e-2 * t26785 + 0.77382407407407407407e-3 * t26787 + 0.77382407407407407407e-3 * t26792 + t27682 * t548 - 0.41270617283950617283e-2 * t19757;
    (t27682, t27685)
}
