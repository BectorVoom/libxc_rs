//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 447/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk447(t203: f64, t2293: f64, t161: f64, t2366: f64, t123: f64, t1570: f64, t4260: f64, t486: f64, t165: f64, t599: f64) -> (f64, f64, f64, f64, f64) {
    let t6417 = t203 * t2293;
    let t6470 = t161 * t2366;
    let t6485 = t1570 * t123;
    let t6507 = t4260 * t486;
    let t6508 = t165 * t599;
    (t6417, t6470, t6485, t6507, t6508)
}
