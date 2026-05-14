//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 871/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk871<F: Float>(t1431: F, t2509: F, t10182: F, t10184: F, t10187: F, t5340: F, t5341: F, t5385: F, t5389: F, t5392: F, t5395: F, t5396: F, t5404: F, t5409: F, t5414: F, t5416: F, t5418: F, t5422: F) -> (F,) {
    let t10615 = t2509 * t1431;
    let t10621 = t5340 + 6.211752672544321 * t5341 + 1.1846959580306418 * t5385 + 0.8091720650647759 * t5389 + 2.427516195194328 * t5392 + t5395 - 4.738783832122567 * t5396 - 7.35994946043302 * t5404 + t5409 - 22.07984838129906 * t10182 - 3.600163427964126 * t10184 + 3.600163427964126 * t10187 + 1.2536914064583544 * t10615 - 6.496391258193384 * t5414 + 0.7380249726277691 * t5416 + 0.8091720650647759 * t5418 + 22.07984838129906 * t5422;
    (t10621,)
}
