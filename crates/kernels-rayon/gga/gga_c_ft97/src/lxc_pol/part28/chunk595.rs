//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 595/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk595(t22907: f64, t25601: f64, t22908: f64, t3204: f64, t1308: f64, t378: f64, t108: f64, t1570: f64, t3188: f64, t1642: f64, t1557: f64, t5618: f64, t984: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25602 = t22907 * t25601;
    let t25605 = t22908 * t3204;
    let t25606 = t22907 * t25605;
    let t25609 = t378 * t1308;
    let t25610 = t108 * t1570;
    let t25611 = t25610 * t3188;
    let t25612 = t25609 * t25611;
    let t25615 = t1642 * t1308;
    let t25616 = t108 * t1557;
    let t25617 = t25616 * t3188;
    let t25618 = t25615 * t25617;
    let t25621 = t5618 * t984;
    (t25602, t25605, t25606, t25609, t25611, t25612, t25615, t25617, t25618, t25621)
}
