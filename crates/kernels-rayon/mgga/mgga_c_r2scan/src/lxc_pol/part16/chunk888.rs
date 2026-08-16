//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 888/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk888(t565: f64, t9463: f64, t1632: f64, t3016: f64, t551: f64, t566: f64, t2573: f64, t8740: f64, t5109: f64, t8756: f64, t2155: f64, t9423: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9469 = t565 * t9463;
    let t9476 = t1632 * t3016;
    let t9477 = t551 * t9476;
    let t9478 = t566 * t9477;
    let t9481 = t8740 * t2573;
    let t9482 = t5109 * t9481;
    let t9485 = t5109 * t8756;
    let t9488 = t2155 * t9423;
    (t9469, t9476, t9478, t9481, t9482, t9485, t9488)
}
