//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1057/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1057(t2354: f64, t41945: f64, t446: f64, t10: f64, t11175: f64, t242: f64, t2366: f64, t89: f64, t9733: f64, t1636: f64, t2344: f64, t2350: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41947 = t446 * t2354 * t41945;
    let t41950 = t10 * t11175 * t242;
    let t41951 = 140.0_f64 / 243.0_f64 * t41950;
    let t41953 = t89 * t9733 * t2366;
    let t41954 = 4.0_f64 / 27.0_f64 * t41953;
    let t41955 = t1636 * t2344;
    let t41957 = t89 * t41955 * t2350;
    (t41947, t41950, t41951, t41953, t41954, t41955, t41957)
}
