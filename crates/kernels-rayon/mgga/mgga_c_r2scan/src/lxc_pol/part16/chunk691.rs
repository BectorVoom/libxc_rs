//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 691/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk691(t5363: f64, t5364: f64, t1726: f64, t1727: f64, t608: f64, t1859: f64, t766: f64, t2: f64, t636: f64, t1758: f64, t188: f64, t1907: f64) -> (f64, f64, f64, f64, f64) {
    let t5366 = 0.5143752e0_f64 * t5363 * t5364;
    let t5373 = t1726 * t608 * t1727;
    let t5375 = t1859 * t766;
    let t5376 = t636 * t2;
    let t5377 = t5376 * t1758;
    let t5378 = t5375 * t5377;
    let t5380 = t1907 * t188;
    (t5366, t5373, t5377, t5378, t5380)
}
