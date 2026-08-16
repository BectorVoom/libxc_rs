//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 231/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk231(t209: f64, t625: f64, t228: f64, t231: f64, t173: f64, t705: f64, t701: f64, t191: f64, t668: f64, t2347: f64, t2360: f64, t703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2432 = t209 * t625;
    let t2434 = t228 * t2432 * t231;
    let t2435 = 0.42562405586419753087e-2_f64 * t2434;
    let t2436 = t173 * t705;
    let t2437 = t701 * t2436;
    let t2440 = 1.0_f64 / t191 / t668;
    let t2441 = t2440 * t2347;
    let t2446 = t703 * t2360;
    (t2434, t2435, t2437, t2440, t2441, t2446)
}
