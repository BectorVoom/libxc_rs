//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1096/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1096(t102: f64, t411: f64, t7914: f64, t1697: f64, t7913: f64, t7919: f64, t1832: f64, t2615: f64, t1844: f64, t2610: f64, t6121: f64, t763: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20396 = 5.84605_f64 * t102 * t7914 * t411;
    let t20397 = t1697 * t7913;
    let t20403 = 70.1526_f64 * t102 * t7919 * t411;
    let t20406 = 52.61445_f64 * t102 * t2615 * t1832;
    let t20409 = 17.53815_f64 * t102 * t1844 * t2610;
    let t20412 = 17.53815_f64 * t102 * t763 * t6121;
    (t20396, t20397, t20403, t20406, t20409, t20412)
}
