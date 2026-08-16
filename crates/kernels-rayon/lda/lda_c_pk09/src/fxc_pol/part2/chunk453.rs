//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 453/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk453(t44: f64, t2225: f64, t2300: f64, t2408: f64, t2437: f64, t7: f64, t2140: f64, t413: f64, t1165: f64, t1173: f64, t1693: f64, t1694: f64, t1695: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t2439 = t2225 + t2300 + t2408 + t2437;
    let t2440 = t7 * t2439;
    let t2444 = piecewise3(t45, 0.0_f64, 2.0_f64 * t44 * t2140);
    let t2445 = t2444 * t413;
    let t2447 = -t1165 + t1693 + t1694 - t1695 + t1173;
    (t2439, t2440, t2444, t2445, t2447)
}
