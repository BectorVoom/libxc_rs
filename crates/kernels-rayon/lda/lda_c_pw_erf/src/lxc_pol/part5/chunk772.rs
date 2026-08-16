//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 772/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk772(t102: f64, t1832: f64, t763: f64, t2619: f64, t411: f64, t2594: f64, t3296: f64, t436: f64, t6121: f64, t120: f64, t2624: f64, t767: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7112 = 11.6921_f64 * t102 * t763 * t1832;
    let t7115 = 5.84605_f64 * t102 * t2619 * t411;
    let t7116 = t3296 * t2594;
    let t7123 = t436 * t6121;
    let t7126 = t120 * t6121;
    let t7128 = 2.923025_f64 * t102 * t7126;
    let t7129 = t2624 * t411;
    let t7133 = t767 * t1832;
    (t7112, t7115, t7116, t7123, t7126, t7128, t7129, t7133)
}
