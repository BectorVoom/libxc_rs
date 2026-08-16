//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 512/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk512(t132: f64, t2396: f64, t2446: f64, t2002: f64, t1885: f64, t222: f64, t343: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t2447 = t2396 + t2446;
    let t2449 = piecewise3(t133, 0.0_f64, t2002);
    let t2454 = t222 * t1885 * t343;
    (t2447, t2449, t2454)
}
