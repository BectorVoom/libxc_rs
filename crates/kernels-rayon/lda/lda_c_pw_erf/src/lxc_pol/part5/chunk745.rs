//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 745/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk745(t1972: f64, t6762: f64, t3965: f64, t5146: f64, t784: f64, t1967: f64, t3967: f64, t494: f64, t6711: f64, t2067: f64, t822: f64, t2443: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6763 = t6762 * t1972;
    let t6765 = 32.0_f64 / 45.0_f64 * t3965 * t6763;
    let t6766 = t5146 * t784;
    let t6767 = t6766 * t1967;
    let t6769 = 16.0_f64 / 27.0_f64 * t3965 * t6767;
    let t6771 = t3967 * t6711 * t494;
    let t6773 = 16.0_f64 / 45.0_f64 * t3965 * t6771;
    let t6776 = 4.0_f64 / 15.0_f64 * t822 * t2067;
    let t6778 = 2.0_f64 / 15.0_f64 * t2443 * t544;
    (t6763, t6765, t6766, t6767, t6769, t6771, t6773, t6776, t6778)
}
