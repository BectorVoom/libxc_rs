//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 612/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk612(t25846: f64, t369: f64, t108: f64, t28: f64, t492: f64, t6547: f64, t6454: f64, t379: f64, t1564: f64, t3238: f64, t5743: f64, t3266: f64, t5675: f64, t8411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25847 = t369 * t25846;
    let t25848 = t25847 * t108;
    let t25849 = t28 * t25848;
    let t25856 = t6547 * t492;
    let t25861 = t6454 * t108;
    let t25862 = t25861 * t379;
    let t25863 = t1564 * t25862;
    let t25867 = t3238 * t5743;
    let t25872 = t8411 * t5675 * t3266;
    (t25847, t25849, t25856, t25861, t25862, t25863, t25867, t25872)
}
