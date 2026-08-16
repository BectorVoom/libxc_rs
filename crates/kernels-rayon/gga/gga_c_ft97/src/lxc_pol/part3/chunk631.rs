//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 631/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk631(t5393: f64, t871: f64, t296: f64, t319: f64, t5299: f64, t840: f64, t1212: f64, t1255: f64, t992: f64, t2875: f64, t2874: f64, t1248: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5394 = t871 * t5393;
    let t5395 = t296 * t5394;
    let t5399 = t840 * t319 * t5299;
    let t5403 = t840 * t1255 * t1212;
    let t5408 = t992 * t1212;
    let t5409 = t2875 * t5408;
    let t5410 = t2874 * t5409;
    let t5413 = t992 * t1248;
    (t5394, t5395, t5399, t5403, t5408, t5409, t5410, t5413)
}
