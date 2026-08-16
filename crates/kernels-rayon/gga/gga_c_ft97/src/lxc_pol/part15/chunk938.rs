//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 938/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk938(t20039: f64, t419: f64, t7705: f64, t173: f64, t20076: f64, t20065: f64, t20069: f64, t20083: f64, t20334: f64, t458: f64, t20341: f64, t1775: f64, t20345: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74126 = t419 * t7705 * t20039;
    let t74143 = t419 * t173 * t20076;
    let t74148 = t419 * t173 * t20065;
    let t74153 = t419 * t173 * t20069;
    let t74162 = t419 * t173 * t20083;
    let t74266 = t458 * t20334;
    let t74268 = t458 * t20341;
    let t74285 = t1775 * t20345;
    (t74126, t74143, t74148, t74153, t74162, t74266, t74268, t74285)
}
