//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 746/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk746(t2425: f64, t595: f64, t2443: f64, t515: f64, t3985: f64, t3988: f64, t3992: f64, t5039: f64, t5055: f64, t5859: f64, t6751: f64, t6755: f64, t6758: f64, t6761: f64, t6765: f64, t6769: f64, t6773: f64, t6776: f64, t6778: f64) -> (f64, f64, f64, f64) {
    let t6780 = 2.0_f64 / 15.0_f64 * t2425 * t595;
    let t6781 = t2443 * t515;
    let t6782 = 4.0_f64 / 45.0_f64 * t6781;
    let t6784 = -t6751 + t6755 - t6758 - t6761 - t6765 + t6769 - t6773 - 2.0_f64 / 27.0_f64 * t3985 - t3988 + t3992 - t6776 - t6778 - t6780 + t6782 + 0.06649088888888889_f64 * t5859 + t5039 - t5055;
    (t6780, t6781, t6782, t6784)
}
