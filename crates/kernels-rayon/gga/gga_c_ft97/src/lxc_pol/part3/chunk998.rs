//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 998/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk998(t1882: f64, t5403: f64, t5399: f64, t5395: f64, t1248: f64, t18: f64, t2882: f64, t2881: f64, t4917: f64, t824: f64, t4265: f64, t2874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19449 = t1882 * t5403;
    let t19451 = t1882 * t5399;
    let t19453 = t1882 * t5395;
    let t19455 = t18 * t1248;
    let t19456 = t2882 * t19455;
    let t19457 = t2881 * t19456;
    let t19460 = t4917 * t824;
    let t19461 = t4265 * t19460;
    let t19462 = t2874 * t19461;
    (t19449, t19451, t19453, t19457, t19460, t19462)
}
