//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 900/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk900(t2344: f64, t2371: f64, t665: f64, t7514: f64, t762: f64, t9895: f64, t2492: f64, t2568: f64, t754: f64, t192: f64, t33300: f64, t2469: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42109 = t2344 * t2371;
    let t42123 = t665 * t7514;
    let t42334 = t9895 * t762;
    let t42339 = t2492 * t2568;
    let t42376 = t9895 * t754;
    let t42500 = t192 * t33300;
    let t42575 = t2492 * t2469;
    (t42109, t42123, t42334, t42339, t42376, t42500, t42575)
}
