//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 903/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk903(t3309: f64, t436: f64, t2: f64, t39: f64, t411: f64, t120: f64, t3318: f64, t1: f64, t119: f64, t2824: f64, t125: f64, t2715: f64, t3310: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8896 = t3309 * t436;
    let t8898 = t2 * t39 * t411;
    let t8899 = t8896 * t8898;
    let t8901 = t3318 * t120;
    let t8902 = t8901 * t8898;
    let t8930 = t2824 * t1 * t119;
    let t8932 = 0.16322666666666666_f64 * t125 * t2715 * t3310 * t8930;
    (t8896, t8899, t8901, t8902, t8930, t8932)
}
