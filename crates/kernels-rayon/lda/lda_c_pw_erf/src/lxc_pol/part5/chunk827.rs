//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 827/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk827(t1440: f64, t7588: f64, t1325: f64, t6988: f64, t799: f64, t2558: f64, t4738: f64, t6991: f64, t833: f64, t1466: f64, t1318: f64, t6997: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7589 = t1440 * t7588;
    let t7591 = 4.0_f64 / 5.0_f64 * t1325 * t7589;
    let t7593 = 8.0_f64 / 15.0_f64 * t6988 * t799;
    let t7595 = 8.0_f64 / 5.0_f64 * t4738 * t2558;
    let t7596 = t6991 * t833;
    let t7597 = t1466 * t7596;
    let t7599 = 4.0_f64 / 5.0_f64 * t1318 * t7597;
    let t7600 = t6997 * t784;
    (t7589, t7591, t7593, t7595, t7596, t7597, t7599, t7600)
}
