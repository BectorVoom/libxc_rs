//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 796/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk796(t2190: f64, t678: f64, t7920: f64, t2160: f64, t49: f64, t7933: f64, t7935: f64, t7490: f64, t7932: f64, t7936: f64, t2185: f64, t7943: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36916 = t2190 * t7920 * t678;
    let t36920 = t2160 * t49;
    let t36922 = t7933 * t36920 * t7935;
    let t36924 = t7490 * t7932;
    let t36925 = t36924 * t7936;
    let t36928 = t7943 * t2185 * t678;
    (t36916, t36920, t36922, t36924, t36925, t36928)
}
