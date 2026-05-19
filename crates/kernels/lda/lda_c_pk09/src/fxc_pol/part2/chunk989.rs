//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 989/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk989<F: Float>(t1431: F, t2509: F, t10182: F, t10184: F, t10187: F, t5340: F, t5341: F, t5385: F, t5389: F, t5392: F, t5395: F, t5396: F, t5404: F, t5409: F, t5414: F, t5416: F, t5418: F, t5422: F) -> F {
    let t10615 = t2509 * t1431;
    let t10621 = t5340 + F::cast_from(6.211752672544321_f64) * t5341 + F::cast_from(1.1846959580306418_f64) * t5385 + F::cast_from(0.8091720650647759_f64) * t5389 + F::cast_from(2.427516195194328_f64) * t5392 + t5395 - F::cast_from(4.738783832122567_f64) * t5396 - F::cast_from(7.35994946043302_f64) * t5404 + t5409 - F::cast_from(22.07984838129906_f64) * t10182 - F::cast_from(3.600163427964126_f64) * t10184 + F::cast_from(3.600163427964126_f64) * t10187 + F::cast_from(1.2536914064583544_f64) * t10615 - F::cast_from(6.496391258193384_f64) * t5414 + F::cast_from(0.7380249726277691_f64) * t5416 + F::cast_from(0.8091720650647759_f64) * t5418 + F::cast_from(22.07984838129906_f64) * t5422;
    t10621
}
