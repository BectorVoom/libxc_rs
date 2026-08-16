//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 721/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk721(t2035: f64, t7292: f64, t2000: f64, t462: f64, t6196: f64, t337: f64, t461: f64, t429: f64, t2042: f64, t1819: f64, t7286: f64, t450: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7293 = t2035 * t7292;
    let t7296 = t462 * t2000;
    let t7297 = t7296 * t6196;
    let t7299 = t461 * t337;
    let t7300 = t7299 * t429;
    let t7301 = t2035 * t7300;
    let t7302 = t7301 * t2042;
    let t7304 = t1819 * t7286;
    let t7307 = t450 * t337;
    (t7293, t7296, t7297, t7300, t7302, t7304, t7307)
}
