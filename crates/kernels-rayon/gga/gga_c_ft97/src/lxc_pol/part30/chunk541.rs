//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 541/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk541(t24330: f64, t6043: f64, t6046: f64, t51: f64, t1410: f64, t695: f64, t3758: f64, t6056: f64, t6055: f64, t444: f64, t6041: f64, t3789: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24332 = t6043 * t24330 * t6046;
    let t24340 = t51 * sigma2;
    let t24345 = t695 * t1410;
    let t24346 = t3758 * t24345;
    let t24357 = t24330 * t6056;
    let t24358 = t6055 * t24357;
    let t24360 = t6041 * t444;
    let t24361 = t3789 * t24360;
    (t24332, t24340, t24345, t24346, t24357, t24358, t24361)
}
