//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 508/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk508(t2482: f64, t888: f64, t9263: f64, t584: f64, t6582: f64) -> (f64, f64) {
    let t9264 = t888 * t2482;
    let t9265 = t9263 * t9264;
    let t9266 = 0.76685851907841499352e0_f64 * t9265;
    let t9267 = t584 * t6582;
    (t9266, t9267)
}
