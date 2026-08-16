//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 788/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk788(t1022: f64, t9356: f64, t1928: f64, t3096: f64, t3094: f64, t5541: f64, t612: f64, t1671: f64, t5544: f64, t2712: f64, t3430: f64, t1044: f64, t640: f64) -> (f64, f64, f64, f64, f64) {
    let t9357 = t1022 * t9356;
    let t9359 = t3096 * t1928;
    let t9360 = t3094 * t9359;
    let t9362 = t5541 * t612;
    let t9363 = t1671 * t5544;
    let t9364 = t9362 * t9363;
    let t9383 = t3096 * t2712;
    let t9384 = t3430 * t9383;
    let t9386 = t640 * t1044;
    (t9357, t9360, t9364, t9384, t9386)
}
