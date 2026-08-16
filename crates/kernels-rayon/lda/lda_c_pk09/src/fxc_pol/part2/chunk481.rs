//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 481/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk481(t2674: f64, t93: f64, t1372: f64, t1374: f64, t1376: f64, t1378: f64, t2502: f64, t2505: f64, t2542: f64, t2546: f64, t1381: f64, t306: f64) -> (f64, f64, f64, f64) {
    let t2675 = t93 * t2674;
    let t2688 = t1372 - 6.25_f64 * t2542 + t1374 + 6.25_f64 * t2546 + t1376 - 1.2466946262544771_f64 * t2502 + t1378 + 1.2466946262544771_f64 * t2505;
    let t2689 = t2688 * t1381;
    let t2690 = t2689 * t306;
    (t2675, t2688, t2689, t2690)
}
