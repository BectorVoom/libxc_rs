//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 622/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk622(t3594: f64, t557: f64, t11: f64, t560: f64, t925: f64, t1361: f64, t325: f64, t1353: f64, t1484: f64, t56: f64, t3590: f64, t174: f64, t205: f64, t3540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3624 = t557 * t3594;
    let t3625 = t11 * t3624;
    let t3627 = t925 * t560;
    let t3629 = t325 * t1361;
    let t3631 = t325 * t1353;
    let t3633 = t56 * t1484;
    let t3634 = t3633 * t3590;
    let t3635 = t11 * t3634;
    let t3638 = t174 * t3540 * t205;
    (t3624, t3625, t3627, t3629, t3631, t3633, t3634, t3635, t3638)
}
