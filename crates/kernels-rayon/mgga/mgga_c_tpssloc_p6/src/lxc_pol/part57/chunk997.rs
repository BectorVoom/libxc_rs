//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 997/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk997(t31376: f64, t5544: f64, t6552: f64, t6637: f64, t101708: f64, t1888: f64, t232: f64, t6646: f64, t101715: f64, t22996: f64, t2632: f64, t121574: f64, t126481: f64, t126484: f64, t126488: f64, t126492: f64, t127917: f64, t1499: f64, t226: f64, t235: f64, t33396: f64) -> f64 {
    let t128001 = t6552 * t6637 * t31376 * t5544;
    let t128007 = t1888 * t6646 * t101708 * t232;
    let t128011 = t1888 * t6646 * t101715 * t232;
    let t128015 = t1888 * t22996 * t101715 * t2632;
    let t128020 = -0.16449340668482264365e-1_f64 * t128001 + t226 * t235 * t127917 - 0.82246703342411321825e-2_f64 * t128007 - 0.82246703342411321825e-2_f64 * t128011 - t126481 + 0.16449340668482264365e-1_f64 * t128015 + t126484 - 0.38381794893125283518e-1_f64 * t121574 - t126488 + t126492 + 2.0_f64 * t1499 * t33396;
    t128020
}
