//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1081/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1081(t7: f64, t132: f64, t10180: f64, t10441: f64, t10480: f64, t10513: f64, t9909: f64, t4214: f64, t849: f64, t222: f64, t4104: f64, t568: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t10516 = piecewise3(t134, 0.0_f64, t10180 + t10441 + t10480 + t10513);
    let t10517 = piecewise3(t8, 0.0_f64, t9909);
    let t10528 = t4214 * t849;
    let t10534 = t222 * t568 * t4104;
    (t10516, t10517, t10528, t10534)
}
