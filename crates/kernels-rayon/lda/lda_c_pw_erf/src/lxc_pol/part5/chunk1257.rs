//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1257/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1257(t186: f64, t211: f64, t22475: f64, t22498: f64, t22522: f64, t22572: f64, t582: f64, t2480: f64, t6851: f64, t22432: f64, t22434: f64, t22436: f64, t22438: f64, t22440: f64, t22442: f64, t22444: f64, t22446: f64, t22448: f64, t22449: f64, t22450: f64) -> (f64, f64, f64) {
    let t22578 = 2.0_f64 / 15.0_f64 * t211 * t186 * t582 * (t22475 + t22498 + t22522 + t22572);
    let t22580 = 4.0_f64 / 5.0_f64 * t6851 * t2480;
    let t22581 = t22432 + t22434 + t22436 - t22438 - t22440 - t22442 - t22444 + t22446 - t22448 - t22449 + t22450 - t22578 + t22580;
    (t22578, t22580, t22581)
}
