//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 734/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk734(t3604: f64, t743: f64, t951: f64, t1349: f64, t11: f64, t1351: f64, t34: f64, t352: f64, t1953: f64, t1943: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4670 = t3604 * t743;
    let t4671 = t4670 * t951;
    let t4672 = t1349 * t4671;
    let t4673 = t11 * t4672;
    let t4675 = t1351 * t34;
    let t4676 = t4675 * t352;
    let t4677 = t1349 * t4676;
    let t4678 = t1953 * t4677;
    let t4680 = t1943 * t954;
    let t4681 = t1349 * t4680;
    let t4682 = t11 * t4681;
    let t4684 = t1943 * t951;
    (t4670, t4671, t4672, t4673, t4675, t4676, t4677, t4678, t4680, t4681, t4682, t4684)
}
