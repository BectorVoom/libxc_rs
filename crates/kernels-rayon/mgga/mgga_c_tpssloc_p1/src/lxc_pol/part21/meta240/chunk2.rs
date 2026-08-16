//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1422/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1422(t1070: f64, t193: f64, t3216: f64, t336: f64, t5691: f64, t5693: f64, t5697: f64, t5729: f64, t5732: f64, t5798: f64, t5800: f64, t5802: f64, t5806: f64, t5810: f64, t5814: f64, t5946: f64, t5950: f64) -> f64 {
    let t5954 = t1070 * t193 * t336 * t5946 - t193 * t3216 * t336 * t5950 - t5691 + t5693 - t5697 + t5729 + t5732 + t5798 + t5800 - t5802 + t5806 - t5810 - t5814;
    t5954
}
