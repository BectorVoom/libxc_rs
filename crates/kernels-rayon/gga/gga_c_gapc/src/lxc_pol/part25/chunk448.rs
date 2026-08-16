//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 448/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk448(t2268: f64, t876: f64, t2261: f64, t770: f64, t640: f64, t769: f64, t791: f64, t4: f64, t891: f64, t2416: f64, t768: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2423 = t2268 * t876;
    let t2426 = t2261 * t770;
    let t2429 = t769 * t640;
    let t2430 = t791 * t2429;
    let t2431 = t891 * t4;
    let t2432 = t2416 * t2431;
    let t2435 = t768 * t825;
    (t2423, t2426, t2430, t2431, t2432, t2435)
}
