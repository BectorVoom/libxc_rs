//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1187/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1187<F: Float>(t1395: F, t21972: F, t1464: F, t16752: F, t2002: F, t11369: F, t1319: F, t6937: F, t11374: F, t1419: F, t21125: F, t5425: F, t16552: F, t21130: F, t21134: F, t531: F, t7141: F) -> (F, F, F, F, F, F, F, F) {
    let t21973 = t1395 * t21972;
    let t21974 = t1464 * t21973;
    let t21976 = t16752 * t2002;
    let t21977 = t1464 * t21976;
    let t21983 = t11369 * t6937 * t1319;
    let t21987 = t11374 * t6937 * t1419;
    let t21990 = t5425 * t21125;
    let t21993 = t16552 * t21130;
    let t21996 = t5425 * t21134;
    let t21999 = t7141 * t531;
    (t21974, t21977, t21983, t21987, t21990, t21993, t21996, t21999)
}
