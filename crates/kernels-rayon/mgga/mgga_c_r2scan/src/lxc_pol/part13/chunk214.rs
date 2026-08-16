//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 214/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk214(t621: f64, t650: f64, t653: f64, t230: f64, t406: f64, t410: f64, t229: f64, t424: f64) -> (f64, f64, f64, f64) {
    let t656 = 0.16081979498692535067e2_f64 * t650 * t653 * t621;
    let t658 = 4.0_f64 * t406 * t230;
    let t660 = 4.0_f64 * t410 * t230;
    let t661 = t424 * t229;
    (t656, t658, t660, t661)
}
