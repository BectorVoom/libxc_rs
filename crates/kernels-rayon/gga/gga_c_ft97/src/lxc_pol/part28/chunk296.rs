//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 296/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk296(t3255: f64, t488: f64, t83: f64, t1882: f64, t955: f64, t1825: f64, t979: f64, t432: f64, t942: f64) -> (f64, f64, f64, f64) {
    let t3256 = t488 * t3255;
    let t3257 = t83 * t3256;
    let t3260 = t1882 * t955;
    let t3262 = t1825 * t979;
    let t3263 = t83 * t3262;
    let t3266 = t942 * t432;
    (t3257, t3260, t3263, t3266)
}
