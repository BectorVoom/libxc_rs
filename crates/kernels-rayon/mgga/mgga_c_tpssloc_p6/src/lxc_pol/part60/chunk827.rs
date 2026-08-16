//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 827/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk827(t5: f64, t29484: f64, t112: f64, t2113: f64, t5456: f64, t1458: f64, t27863: f64, t28001: f64, t28004: f64, t28006: f64, t28009: f64, t28011: f64, t28019: f64, t5493: f64, t7266: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t29485 = piecewise3(t8, 0.0_f64, t29484);
    let t29486 = t29485 * t112;
    let t29493 = t2113 * t5456;
    let t29497 = 4.0_f64 * t1458 * t27863 + 2.0_f64 * t5493 * t7266 + t28001 + t28004 + t28006 + t28009 + t28011 + t28019 + t29486 + 2.0_f64 * t29493;
    (t29485, t29486, t29493, t29497)
}
