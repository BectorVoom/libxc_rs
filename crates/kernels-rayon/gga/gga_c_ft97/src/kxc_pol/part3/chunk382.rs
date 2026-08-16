//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 382/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk382(t235: f64, t693: f64, t226: f64, t209: f64, t625: f64, t228: f64, t231: f64, t173: f64, t705: f64, t701: f64, t191: f64, t668: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2426 = 1.0_f64 / t693 / t235;
    let t2427 = t226 * t2426;
    let t2432 = t209 * t625;
    let t2434 = t228 * t2432 * t231;
    let t2435 = 0.42562405586419753087e-2_f64 * t2434;
    let t2436 = t173 * t705;
    let t2437 = t701 * t2436;
    let t2440 = 1.0_f64 / t191 / t668;
    (t2426, t2427, t2434, t2435, t2436, t2437, t2440)
}
