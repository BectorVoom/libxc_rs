//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 558/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk558(t6275: f64, t8392: f64, t312: f64, t6260: f64, t1483: f64, t8232: f64, t1882: f64, t6284: f64, t6293: f64, t2680: f64, t6308: f64, t6310: f64, t681: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24903 = t8392 * t6275;
    let t24908 = t312 * t6260;
    let t24955 = 4.0_f64 / 27.0_f64 * t8232 * t1483;
    let t24960 = t1882 * t6284;
    let t24962 = t1882 * t6293;
    let t24964 = t2680 * t6260;
    let t24974 = t6308 * t681 * t6310;
    (t24903, t24908, t24955, t24960, t24962, t24964, t24974)
}
