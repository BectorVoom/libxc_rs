//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1133/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1133(t1498: f64, t1980: f64, t1983: f64, t13444: f64, t13447: f64, t13450: f64, t13453: f64, t13455: f64, t13456: f64, t13457: f64, t13461: f64, t13463: f64, t13465: f64, t13467: f64) -> (f64, f64) {
    let t13470 = 2.0_f64 / 15.0_f64 * t1498 * t1980 * t1983;
    let t13471 = t13444 + t13447 / 3.0_f64 + 0.18233333333333332_f64 * t13450 + t13453 - t13455 + t13456 + t13457 + t13461 + t13463 + t13465 + t13467 - t13470;
    (t13470, t13471)
}
