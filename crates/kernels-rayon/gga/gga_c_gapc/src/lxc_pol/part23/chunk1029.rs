//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1029/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1029(t2902: f64, t760: f64, t103: f64, t786: f64, t9740: f64, t2763: f64, t6148: f64, t2920: f64, t147: f64, t19: f64, t2299: f64, t3296: f64) -> (f64, f64, f64, f64, f64) {
    let t24095 = t2902 * t760;
    let t24110 = t9740 * t103 * t786;
    let t24132 = t6148 * t2763;
    let t24181 = t2920 * t760;
    let t24195 = t3296 * t2299 * t19 * t147;
    (t24095, t24110, t24132, t24181, t24195)
}
