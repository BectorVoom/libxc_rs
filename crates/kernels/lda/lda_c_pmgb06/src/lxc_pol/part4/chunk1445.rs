//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1445/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1445<F: Float>(t10494: F, t1338: F, t18418: F, t18420: F, t18422: F, t18424: F, t18426: F, t18428: F, t18430: F, t18432: F, t18434: F, t18437: F, t2422: F, t399: F, t4435: F, t6939: F, t795: F, t84: F) -> F {
    let t18440 = F::new(1.0051538464260528) * t10494 - F::new(0.1675256410710088) * t795 * t4435 - F::new(0.0837628205355044) * t1338 * t2422 - F::new(0.1675256410710088) * t399 * t6939 - F::new(0.3350512821420176) * t18418 - F::new(0.1675256410710088) * t18420 - F::new(0.1675256410710088) * t18422 + F::new(0.3350512821420176) * t18424 + F::new(0.1675256410710088) * t18426 + F::new(0.1675256410710088) * t18428 + F::new(0.1675256410710088) * t18430 + F::new(0.1675256410710088) * t18432 + F::new(0.3350512821420176) * t18434 - F::new(0.0837628205355044) * t84 * t18437;
    t18440
}
