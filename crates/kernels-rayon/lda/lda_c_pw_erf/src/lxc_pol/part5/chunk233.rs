//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 233/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk233(t265: f64, t668: f64, t256: f64, t267: f64, t498: f64, t513: f64, t517: f64, t527: f64, t546: f64, t553: f64, t567: f64, t570: f64, t579: f64, t597: f64, t640: f64, t649: f64, t655: f64, t658: f64, t665: f64) -> (f64, f64) {
    let t670 = 2.0_f64 / 45.0_f64 * t265 * t668;
    let t671 = t498 + t513 + t517 + t527 - t546 + t553 + t567 + t570 + t579 - t597 + t640 * t256 / 3.0_f64 + t649 + t655 + t658 - t665 * t267 / 15.0_f64 - t670;
    (t670, t671)
}
