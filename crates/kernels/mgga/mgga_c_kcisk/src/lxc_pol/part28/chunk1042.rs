//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1042/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1042<F: Float>(t15936: F, t6707: F, t1800: F, t1869: F, t4811: F, t8674: F, t6702: F, t6965: F, t1873: F, t8678: F, t5074: F, t8951: F, t1894: F, t8672: F, t1801: F, t11227: F) -> (F, F, F, F, F, F, F) {
    let t23965 = t15936 * t6707;
    let t23966 = t1800 * t23965;
    let t23967 = t1869 * t23966;
    let t23969 = t4811 * t8674;
    let t23971 = t6965 * t6702;
    let t23972 = t1873 * t23971;
    let t23973 = t1869 * t23972;
    let t23976 = t4811 * t8678;
    let t23978 = t5074 * t8951;
    let t23980 = t8672 * t1894;
    let t23981 = t1801 * t23980;
    let t23982 = t11227 * t23981;
    (t23967, t23969, t23973, t23976, t23978, t23980, t23982)
}
