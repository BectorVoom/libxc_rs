//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1037/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1037(t10126: f64, t13095: f64, t13096: f64, t13098: f64, t13102: f64, t13103: f64, t13105: f64, t13106: f64, t13108: f64, t1484: f64, t2522: f64, t2523: f64, t4119: f64, t9789: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64) -> f64 {
    let t13483 = 3.0_f64 * t10126 * t1484 * t2522 + 6.0_f64 * t2522 * t2523 * t4119 + t13095 + t13096 + t13098 + t13102 + t13103 + t13105 + t13106 - t13108 - t9789 + t9793 + t9797 - t9820 - t9824 - t9876 - t9884 + t9887 + t9890;
    t13483
}
