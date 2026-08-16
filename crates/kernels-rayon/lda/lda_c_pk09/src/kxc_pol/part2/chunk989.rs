//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 989/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk989(t1431: f64, t2509: f64, t10182: f64, t10184: f64, t10187: f64, t5340: f64, t5341: f64, t5385: f64, t5389: f64, t5392: f64, t5395: f64, t5396: f64, t5404: f64, t5409: f64, t5414: f64, t5416: f64, t5418: f64, t5422: f64) -> f64 {
    let t10615 = t2509 * t1431;
    let t10621 = t5340 + 6.211752672544321_f64 * t5341 + 1.1846959580306418_f64 * t5385 + 0.8091720650647759_f64 * t5389 + 2.427516195194328_f64 * t5392 + t5395 - 4.738783832122567_f64 * t5396 - 7.35994946043302_f64 * t5404 + t5409 - 22.07984838129906_f64 * t10182 - 3.600163427964126_f64 * t10184 + 3.600163427964126_f64 * t10187 + 1.2536914064583544_f64 * t10615 - 6.496391258193384_f64 * t5414 + 0.7380249726277691_f64 * t5416 + 0.8091720650647759_f64 * t5418 + 22.07984838129906_f64 * t5422;
    t10621
}
