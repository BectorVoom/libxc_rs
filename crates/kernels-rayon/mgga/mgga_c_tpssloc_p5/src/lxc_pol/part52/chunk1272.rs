//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1272/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1272(t225: f64, t32878: f64, t112680: f64, t112686: f64, t112702: f64, t30713: f64, t4166: f64, t30716: f64, t112797: f64, t32844: f64, t13242: f64, t232: f64, t30714: f64, t4180: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t118510 = t32878 * t225;
    let t118518 = 0.76763589786250567036e-1_f64 * t112680;
    let t118523 = 0.76763589786250567036e-1_f64 * t112686;
    let t118526 = 0.16449340668482264365e-1_f64 * t112702;
    let t118532 = t4166 * t30713;
    let t118533 = t118532 * t30716;
    let t118535 = t112797 * t32844;
    let t118539 = t30714 * t4180 * t13242 * t232;
    (t118510, t118518, t118523, t118526, t118533, t118535, t118539)
}
