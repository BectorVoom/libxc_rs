//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1445/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1445<F: Float>(t10494: F, t1338: F, t18418: F, t18420: F, t18422: F, t18424: F, t18426: F, t18428: F, t18430: F, t18432: F, t18434: F, t18437: F, t2422: F, t399: F, t4435: F, t6939: F, t795: F, t84: F) -> F {
    let t18440 = F::cast_from(1.0051538464260528_f64) * t10494 - F::cast_from(0.1675256410710088_f64) * t795 * t4435 - F::cast_from(0.0837628205355044_f64) * t1338 * t2422 - F::cast_from(0.1675256410710088_f64) * t399 * t6939 - F::cast_from(0.3350512821420176_f64) * t18418 - F::cast_from(0.1675256410710088_f64) * t18420 - F::cast_from(0.1675256410710088_f64) * t18422 + F::cast_from(0.3350512821420176_f64) * t18424 + F::cast_from(0.1675256410710088_f64) * t18426 + F::cast_from(0.1675256410710088_f64) * t18428 + F::cast_from(0.1675256410710088_f64) * t18430 + F::cast_from(0.1675256410710088_f64) * t18432 + F::cast_from(0.3350512821420176_f64) * t18434 - F::cast_from(0.0837628205355044_f64) * t84 * t18437;
    t18440
}
