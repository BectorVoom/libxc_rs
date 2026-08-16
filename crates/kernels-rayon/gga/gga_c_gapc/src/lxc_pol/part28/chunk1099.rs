//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1099/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1099(t103: f64, t2669: f64, t2315: f64, t2598: f64, t2207: f64, t640: f64, t1645: f64, t268: f64, t2299: f64, t830: f64, t6856: f64, t11925: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t22442 = t2669 * t103;
    let t22581 = t2598 * t2315;
    let t22657 = t2207 * t640;
    let t22672 = t1645 * t268;
    let t22783 = t830 * t2299;
    let t22851 = t6856 * t103;
    let t22866 = t11925 * pi * t875;
    (t22442, t22581, t22657, t22672, t22783, t22851, t22866)
}
