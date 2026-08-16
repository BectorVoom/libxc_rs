//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 784/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk784(t86: f64, t32390: f64, t32649: f64, t113: f64, t5: f64, t505: f64, t7293: f64, t5764: f64, t7150: f64, t1374: f64, t1774: f64, t7298: f64, t1360: f64, t379: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87 = 10000000.0_f64 <= t86;
    let t32650 = t32390 + t32649;
    let t32657 = piecewise3(t87, 0.0_f64, t5 * t32650 * t113 / 4.0_f64 + t5 * t7293 * t505 / 4.0_f64);
    let t32658 = t5764 * t7150;
    let t32661 = t1774 * t1374;
    let t32663 = t7298 * t32661 / 18.0_f64;
    let t32664 = t1360 * t379;
    (t32650, t32657, t32658, t32661, t32663, t32664)
}
