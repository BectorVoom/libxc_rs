//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 509/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk509(t1318: f64, t2532: f64, t2263: f64, t2463: f64, t2470: f64, t2475: f64, t2477: f64, t2482: f64, t2501: f64, t2503: f64, t2507: f64, t2509: f64, t2511: f64, t2530: f64, t256: f64) -> (f64, f64) {
    let t2534 = 8.0_f64 / 15.0_f64 * t1318 * t2532;
    let t2535 = 4.0_f64 / 9.0_f64 * t2263 + t2463 * t256 / 3.0_f64 + t2470 + t2475 + t2477 + t2482 - t2501 + t2503 + t2507 - t2509 - t2511 - t2530 - t2534;
    (t2534, t2535)
}
