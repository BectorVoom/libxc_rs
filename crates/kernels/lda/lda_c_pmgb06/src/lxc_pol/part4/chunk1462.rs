//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1462/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1462<F: Float>(t11206: F, t11465: F, t1296: F, t1297: F, t1309: F, t18591: F, t18632: F, t18641: F, t18663: F, t18690: F, t18702: F, t18723: F, t18761: F, t2241: F, t2722: F, t2730: F, t3625: F, t3632: F, t384: F, t5834: F, t5843: F, t5846: F, t5849: F, t5880: F, t7053: F, t7056: F, t7060: F, t7086: F, t74: F, t787: F, t8404: F) -> F {
    let t18785 = -F::cast_from(6.0_f64) * t3632 * t2730 * t1297 - F::cast_from(6.0_f64) * t3632 * t2722 * t1309 + F::cast_from(8.0_f64) * t5834 * t5846 + F::cast_from(4.0_f64) * t5834 * t5849 + (t18591 + t18632 + t18641 + t18663 + t18690 + t18702 + t18723 + t18761) * t74 + F::cast_from(4.0_f64) * t1296 * t787 * t5880 + F::cast_from(4.0_f64) * t3625 * t7060 + F::cast_from(4.0_f64) * t1296 * t7086 * t384 + F::cast_from(2.0_f64) * t1296 * t2730 * t1309 + F::cast_from(8.0_f64) * t11206 * t2241 - F::cast_from(12.0_f64) * t11465 * t5843 - F::cast_from(12.0_f64) * t8404 * t7053 + F::cast_from(8.0_f64) * t3625 * t7056;
    t18785
}
