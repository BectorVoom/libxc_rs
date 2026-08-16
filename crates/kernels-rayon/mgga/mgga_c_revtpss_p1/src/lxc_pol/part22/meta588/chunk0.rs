//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2461/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2461(t1587: f64, t2: f64, t580: f64, t11506: f64, t6189: f64, t11509: f64, t972: f64, t981: f64, t11144: f64, t5819: f64, t606: f64, t11142: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18890 = t1587 * t2;
    let t18892 = 2.0_f64 * t18890 * t580;
    let t18898 = t11506 * t6189;
    let t18899 = t11509 * t972;
    let t18900 = t18898 * t18899;
    let t18902 = 0.10254018858216406658e4_f64 * t981 * t18900;
    let t18903 = t11144 * t5819;
    let t18904 = t18903 * t606;
    let t18905 = t11142 * t18904;
    (t18890, t18892, t18898, t18900, t18902, t18903, t18904, t18905)
}
