//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 417/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk417(t108: f64, t6455: f64, t28: f64, t1308: f64, t984: f64, t5630: f64, t925: f64, t1902: f64, t1307: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6456 = t6455 * t108;
    let t6457 = t28 * t6456;
    let t6460 = t1308 * t984;
    let t6461 = t28 * t6460;
    let t6465 = t5630 * t925;
    let t6466 = t1902 * t6465;
    let t6469 = t1307 * t942;
    (t6456, t6457, t6460, t6461, t6465, t6466, t6469)
}
