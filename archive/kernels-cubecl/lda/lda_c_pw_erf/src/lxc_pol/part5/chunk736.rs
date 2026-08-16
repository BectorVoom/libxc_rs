//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 736/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk736<F: Float>(t1953: F, t6646: F, t2416: F, t325: F, t557: F, t6413: F, t11: F, t6446: F, t2420: F, t1349: F, t6366: F, t6361: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6647 = t1953 * t6646;
    let t6649 = t325 * t2416;
    let t6651 = t557 * t6413;
    let t6652 = t11 * t6651;
    let t6654 = t557 * t6446;
    let t6655 = t1953 * t6654;
    let t6657 = t325 * t2420;
    let t6659 = t1349 * t6366;
    let t6660 = t11 * t6659;
    let t6662 = t557 * t6361;
    (t6647, t6649, t6651, t6652, t6654, t6655, t6657, t6659, t6660, t6662)
}
