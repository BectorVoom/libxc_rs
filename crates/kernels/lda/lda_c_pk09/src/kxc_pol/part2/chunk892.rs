//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 892/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk892<F: Float>(t1701: F, t2707: F, t1161: F, t6360: F, t6376: F, t6381: F, t1705: F, t4861: F, t9673: F, t9674: F, t9675: F, t1702: F, t10974: F, t10977: F, t10980: F, t10984: F, t10987: F, t1713: F, t253: F, t6347: F, t6349: F, t6350: F, t6352: F, t6356: F, t6358: F, t6363: F, t6367: F) -> (F,) {
    let t10989 = t1701 * t2707;
    let t10990 = t10989 * t1161;
    let t10991 = t6360 * t10990;
    let t10993 = t6376 * t2707;
    let t10996 = t6381 * t2707;
    let t10997 = t10996 * t1705;
    let t11000 = t9673 + t9674 - t9675 - t4861;
    let t11001 = t1702 * t11000;
    let t11004 = t6347 - t6349 + 1.28 * t6350 - 1.28 * t6352 + t6356 - 1.28 * t6358 + 1.28 * t6363 - t6367 + 1.28 * t10974 - 1.28 * t10977 + 1.28 * t253 * t10980 - 1.28 * t253 * t10984 - 1.28 * t10987 + 1.28 * t10991 - 1.28 * t253 * t10993 + 2.56 * t1713 * t10997 - 1.28 * t253 * t11001;
    (t11004,)
}
