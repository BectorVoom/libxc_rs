//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 743/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk743(t4522: f64, t6733: f64, t1996: f64, t4479: f64, t2010: f64, t4475: f64, t1325: f64, t3965: f64, t3974: f64, t4488: f64, t4506: f64, t4948: f64, t6690: f64, t6693: f64, t6697: f64, t6700: f64, t6703: f64, t6706: f64, t6708: f64, t6713: f64, t6717: f64, t6720: f64, t6725: f64, t6730: f64, t6734: f64) -> (f64, f64, f64, f64) {
    let t6737 = t4522 * t6733;
    let t6740 = t4479 * t1996;
    let t6743 = t4475 * t2010;
    let t6746 = -16.0_f64 / 45.0_f64 * t6690 - 8.0_f64 / 15.0_f64 * t1325 * t6693 + 8.0_f64 / 135.0_f64 * t6697 + 8.0_f64 / 81.0_f64 * t6700 + 8.0_f64 / 135.0_f64 * t6703 + 8.0_f64 / 81.0_f64 * t6706 + 16.0_f64 / 135.0_f64 * t6708 - t4948 + 16.0_f64 / 45.0_f64 * t4488 * t6713 + 16.0_f64 / 45.0_f64 * t4488 * t6717 - 8.0_f64 / 27.0_f64 * t4488 * t6720 - 16.0_f64 / 45.0_f64 * t3974 * t6725 + 16.0_f64 / 45.0_f64 * t4506 * t6730 + 16.0_f64 / 45.0_f64 * t4506 * t6734 - 8.0_f64 / 27.0_f64 * t4506 * t6737 - 16.0_f64 / 45.0_f64 * t3965 * t6740 - 16.0_f64 / 45.0_f64 * t3974 * t6743;
    (t6737, t6740, t6743, t6746)
}
