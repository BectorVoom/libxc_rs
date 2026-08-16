//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1153/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1153(t15852: f64, t739: f64, t1326: f64, t519: f64, t34: f64, t6330: f64, t4829: f64, t1446: f64, t7698: f64, t15867: f64, t1991: f64, t21159: f64, t21161: f64, t21165: f64, t21169: f64, t21173: f64, t21175: f64, t21179: f64, t21183: f64, t21185: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21186 = t15852 * t739;
    let t21189 = 8.0_f64 / 15.0_f64 * t519 * t1326 * t21186;
    let t21190 = t6330 * t34;
    let t21193 = 16.0_f64 / 15.0_f64 * t519 * t4829 * t21190;
    let t21195 = 4.0_f64 / 9.0_f64 * t1446 * t7698;
    let t21196 = t15867 * t739;
    let t21199 = 4.0_f64 / 9.0_f64 * t519 * t1991 * t21196;
    let t21200 = -t21159 - t21161 - t21165 - t21169 + t21173 - t21175 - t21179 - t21183 - t21185 - t21189 - t21193 + t21195 + t21199;
    (t21186, t21189, t21190, t21193, t21195, t21196, t21199, t21200)
}
