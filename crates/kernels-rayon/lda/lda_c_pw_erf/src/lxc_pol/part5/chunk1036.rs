//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1036/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1036(t3854: f64, t571: f64, t6361: f64, t4794: f64, t6366: f64, t211: f64, t514: f64, t6844: f64, t1446: f64, t6233: f64, t3802: f64, t519: f64, t6460: f64) -> (f64, f64, f64, f64, f64) {
    let t18314 = t571 * t3854 * t6361;
    let t18317 = t571 * t4794 * t6366;
    let t18390 = t211 * t514 * t6844;
    let t18404 = t1446 * t6233;
    let t18407 = t519 * t3802 * t6460;
    (t18314, t18317, t18390, t18404, t18407)
}
