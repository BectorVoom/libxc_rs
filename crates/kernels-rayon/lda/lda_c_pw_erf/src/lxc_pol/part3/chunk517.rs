//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 517/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk517(t34: f64, t523: f64, t2176: f64, t519: f64, t529: f64, t806: f64, t494: f64, t1440: f64, t1325: f64, t1390: f64, t542: f64, t581: f64, t811: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2177 = t523 * t34;
    let t2178 = t2176 * t2177;
    let t2180 = 8.0_f64 / 45.0_f64 * t519 * t2178;
    let t2181 = t529 * t806;
    let t2182 = t2181 * t494;
    let t2183 = t1440 * t2182;
    let t2185 = 4.0_f64 / 15.0_f64 * t1325 * t2183;
    let t2186 = t1390 * t806;
    let t2187 = t2186 * t542;
    let t2188 = t1440 * t2187;
    let t2190 = 4.0_f64 / 15.0_f64 * t519 * t2188;
    let t2191 = t581 * t811;
    (t2177, t2178, t2180, t2181, t2182, t2183, t2185, t2186, t2187, t2188, t2190, t2191)
}
