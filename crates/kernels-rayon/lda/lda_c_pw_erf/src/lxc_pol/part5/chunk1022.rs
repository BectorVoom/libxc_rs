//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1022/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1022(t2104: f64, t6215: f64, t1284: f64, t514: f64, t548: f64, t6866: f64, t2137: f64, t5211: f64, t1518: f64, t185: f64, t2498: f64, t1318: f64, t3899: f64, t6992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17107 = t2104 * t6215;
    let t17109 = t1284 * t6215;
    let t17112 = t548 * t514 * t6866;
    let t17114 = t5211 * t2137;
    let t17117 = t185 * t1518 * t2498;
    let t17123 = t1318 * t3899 * t6992;
    (t17107, t17109, t17112, t17114, t17117, t17123)
}
