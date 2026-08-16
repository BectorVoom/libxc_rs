//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 452/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk452(t2354: f64, t89: f64, t143: f64, t151: f64, t155: f64, t179: f64, t2210: f64, t2214: f64, t2419: f64, t886: f64, t888: f64, t921: f64, t925: f64, t946: f64, t959: f64, t98: f64, t982: f64, t986: f64) -> (f64, f64) {
    let t2426 = t2354 * t89;
    let t2437 = -t886 + t888 + t921 + t925 - 1.8805371096875316_f64 * t151 * t2214 + 1.8805371096875316_f64 * t2419 * t98 - 19.489173774580152_f64 * t155 * t2210 - 19.489173774580152_f64 * t155 * t2214 + 19.489173774580152_f64 * t2426 * t98 + 3.7610742193750633_f64 * t143 * t2210 + 3.7610742193750633_f64 * t143 * t2214 - 18.635258017632964_f64 * t179 * t2210 - 18.635258017632964_f64 * t179 * t2214 - t946 + t959 - t982 + t986;
    (t2426, t2437)
}
