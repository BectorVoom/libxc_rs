//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 805/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk805(t12844: f64, t4441: f64, t4439: f64, t4425: f64, t4446: f64, t1599: f64, t1590: f64, t609: f64, t4313: f64, t622: f64, t1615: f64, t4314: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12845 = t12844 * t4441;
    let t12846 = t4439 * t12845;
    let t12848 = t4425 * t4446;
    let t12849 = t1599 * t12848;
    let t12856 = t1590 * t1590;
    let t12857 = 1.0_f64 / t12856;
    let t12858 = t609 * t12857;
    let t12861 = 1.0_f64 / t4313 / t622;
    let t12886 = t1615 * t4314;
    (t12846, t12849, t12856, t12857, t12858, t12861, t12886)
}
