//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 442/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk442(t168: f64, t2152: f64, t96: f64, t1029: f64, t1030: f64, t1034: f64, t1035: f64, t2159: f64, t2163: f64, t2167: f64, t2171: f64, t2175: f64, t2179: f64) -> (f64, f64, f64) {
    let t2318 = t2152 * t168;
    let t2319 = t96 * t2318;
    let t2335 = t1029 + t1030 + 4.59690841536205_f64 * t2159 + 4.59690841536205_f64 * t2163 - 4.59690841536205_f64 * t2167 + t1034 + t1035 + 0.3056501876701794_f64 * t2171 + 0.3056501876701794_f64 * t2175 - 0.3056501876701794_f64 * t2179;
    (t2318, t2319, t2335)
}
