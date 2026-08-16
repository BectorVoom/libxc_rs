//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 830/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk830(t164: f64, t179: f64, t2305: f64, t4021: f64, t4028: f64, t744: f64, t8096: f64, t8101: f64, t8394: f64, t8404: f64, t8407: f64, t8413: f64, t8416: f64, t949: f64, t953: f64) -> f64 {
    let t8419 = 2.400108951976084_f64 * t4021 - 3.2915558116322368_f64 * t4028 + 1.2536914064583544_f64 * t8394 + 1.2536914064583544_f64 * t2305 * t744 + 1.2536914064583544_f64 * t2305 * t949 - 1.2536914064583544_f64 * t2305 * t953 - 0.04115066352984959_f64 * t164 * t8404 - 18.635258017632964_f64 * t8407 - 18.635258017632964_f64 * t179 * t8096 - 18.635258017632964_f64 * t179 * t8101 + 2.427516195194328_f64 * t8413 + 0.04115066352984959_f64 * t164 * t8416;
    t8419
}
