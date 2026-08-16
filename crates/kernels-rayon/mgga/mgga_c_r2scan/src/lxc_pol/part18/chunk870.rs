//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 870/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk870(t3179: f64, t780: f64, t113: f64, t8735: f64, t6086: f64, t6085: f64, t8740: f64) -> (f64, f64, f64, f64) {
    let t9240 = t3179 * t780;
    let t9242 = t8735 * t113;
    let t9243 = t6086 * t9242;
    let t9244 = t6085 * t9243;
    let t9246 = t8740 * t113;
    (t9240, t9242, t9244, t9246)
}
