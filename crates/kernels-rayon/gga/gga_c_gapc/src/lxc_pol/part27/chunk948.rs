//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 948/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk948(t11543: f64, t11546: f64, t116: f64, t655: f64, t3163: f64, t3691: f64, t3696: f64, t3703: f64, t424: f64, t134: f64, t3698: f64, t3702: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11547 = t11543 * t11546;
    let t11549 = t116 * t655;
    let t11550 = t11549 * t11546;
    let t11552 = t3691 * t3163;
    let t11555 = t424 * t3696 * t3703;
    let t11557 = t3698 * t134;
    let t11558 = t11557 * t3702;
    (t11547, t11549, t11550, t11552, t11555, t11557, t11558)
}
