//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1227/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1227<F: Float>(t1506: F, t22714: F, t12861: F, t7509: F, t1615: F, t12858: F, t15800: F, t15826: F, t18253: F, t20875: F, t20880: F, t20885: F, t20889: F, t20892: F, t20894: F, t20898: F, t20900: F, t20902: F, t20908: F, t20910: F, t20912: F, t20923: F, t20928: F, t20932: F, t20937: F, t6208: F) -> (F, F, F) {
    let t22715 = t1506 * t22714;
    let t22722 = t7509 * t12861;
    let t22723 = t22722 * t1615;
    let t22740 = -0.11607361111111111111e-2 * t20875 + 0.77382407407407407407e-3 * t20880 - 0.23214722222222222222e-2 * t20885 + 0.19345601851851851852e-2 * t20889 + 0.77382407407407407407e-3 * t15800 + 0.15476481481481481481e-2 * t20892 - 0.178244852896875e-2 * t12858 * t22723 - 0.15476481481481481481e-2 * t20894 - 0.23214722222222222221e-2 * t20898 - 0.41270617283950617283e-2 * t20900 + 0.77382407407407407407e-3 * t20902 - 0.51588271604938271603e-3 * t15826 + 0.77382407407407407407e-3 * t20908 - 0.23214722222222222222e-2 * t20910 + 0.15476481481481481481e-2 * t20912 - 0.92858888888888888886e-2 * t20923 + 0.178089025e-1 * t18253 * t6208 - 0.11607361111111111111e-2 * t20928 - 0.23214722222222222222e-2 * t20932 - 0.23214722222222222222e-2 * t20937;
    (t22715, t22723, t22740)
}
