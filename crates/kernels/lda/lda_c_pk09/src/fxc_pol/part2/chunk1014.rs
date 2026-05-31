//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1014/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1014<F: Float>(t11000: F, t1702: F, t10974: F, t10977: F, t10980: F, t10984: F, t10987: F, t10991: F, t10993: F, t10997: F, t1713: F, t253: F, t6347: F, t6349: F, t6350: F, t6352: F, t6356: F, t6358: F, t6363: F, t6367: F) -> F {
    let t11001 = t1702 * t11000;
    let t11004 = t6347 - t6349 + F::cast_from(1.28_f64) * t6350 - F::cast_from(1.28_f64) * t6352 + t6356 - F::cast_from(1.28_f64) * t6358 + F::cast_from(1.28_f64) * t6363 - t6367 + F::cast_from(1.28_f64) * t10974 - F::cast_from(1.28_f64) * t10977 + F::cast_from(1.28_f64) * t253 * t10980 - F::cast_from(1.28_f64) * t253 * t10984 - F::cast_from(1.28_f64) * t10987 + F::cast_from(1.28_f64) * t10991 - F::cast_from(1.28_f64) * t253 * t10993 + F::cast_from(2.56_f64) * t1713 * t10997 - F::cast_from(1.28_f64) * t253 * t11001;
    t11004
}
