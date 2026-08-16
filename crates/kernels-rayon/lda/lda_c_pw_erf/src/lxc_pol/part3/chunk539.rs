//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 539/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk539(t2760: f64, t1553: f64, t452: f64, t405: f64, t137: f64, t142: f64) -> (f64, f64, f64, f64) {
    let t2761 = 12.0_f64 * t2760;
    let t2763 = t452 * t1553;
    let t2764 = t405 * t2763;
    let t2765 = t137 * t142;
    (t2761, t2763, t2764, t2765)
}
