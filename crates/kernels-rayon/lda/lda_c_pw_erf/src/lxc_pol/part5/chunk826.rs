//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 826/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk826(t6944: f64, t784: f64, t1440: f64, t1325: f64, t2146: f64, t2540: f64, t2544: f64, t6963: f64, t811: f64, t1466: f64, t1318: f64, t6979: f64, t806: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7576 = t6944 * t784;
    let t7577 = t1440 * t7576;
    let t7579 = 8.0_f64 / 5.0_f64 * t1325 * t7577;
    let t7581 = 4.0_f64 / 15.0_f64 * t2146 * t2540;
    let t7583 = 4.0_f64 / 9.0_f64 * t2146 * t2544;
    let t7584 = t6963 * t811;
    let t7585 = t1466 * t7584;
    let t7587 = 8.0_f64 / 5.0_f64 * t1318 * t7585;
    let t7588 = t6979 * t806;
    (t7576, t7577, t7579, t7581, t7583, t7584, t7585, t7587, t7588)
}
