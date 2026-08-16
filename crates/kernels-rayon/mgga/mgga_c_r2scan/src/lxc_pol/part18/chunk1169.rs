//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1169/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1169(t10615: f64, t12383: f64, t3275: f64, t10776: f64, t10810: f64, t3115: f64, t3295: f64, t9540: f64, t9517: f64, t3308: f64, t37965: f64, t8821: f64) -> (f64, f64, f64, f64, f64) {
    let t42976 = 5.0_f64 / 8.0_f64 * t3275 * t10615 * t12383;
    let t42978 = t10776 * t10810 * t3115;
    let t42980 = t3295 * t9540;
    let t42982 = t3295 * t9517;
    let t42985 = t37965 * t3308 * t8821;
    (t42976, t42978, t42980, t42982, t42985)
}
