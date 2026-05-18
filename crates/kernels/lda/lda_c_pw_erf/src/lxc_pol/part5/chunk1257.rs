//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1257/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1257<F: Float>(t186: F, t211: F, t22475: F, t22498: F, t22522: F, t22572: F, t582: F, t2480: F, t6851: F, t22432: F, t22434: F, t22436: F, t22438: F, t22440: F, t22442: F, t22444: F, t22446: F, t22448: F, t22449: F, t22450: F) -> (F, F, F) {
    let t22578 = F::new(2.0) / F::new(15.0) * t211 * t186 * t582 * (t22475 + t22498 + t22522 + t22572);
    let t22580 = F::new(4.0) / F::new(5.0) * t6851 * t2480;
    let t22581 = t22432 + t22434 + t22436 - t22438 - t22440 - t22442 - t22444 + t22446 - t22448 - t22449 + t22450 - t22578 + t22580;
    (t22578, t22580, t22581)
}
