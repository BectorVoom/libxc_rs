//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1028/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1028(t186: f64, t2153: f64, t2206: f64, t2389: f64, t2211: f64, t2299: f64, t2404: f64, t2546: f64, t122: f64, t188: f64, t311: f64, t6851: f64) -> (f64, f64, f64, f64, f64) {
    let t18856 = t2153 * t186;
    let t18866 = t2389 * t2206;
    let t19048 = t2211 * t2299;
    let t19055 = t2546 * t2404;
    let t19094 = t311 * t6851 * t122 * t188;
    (t18856, t18866, t19048, t19055, t19094)
}
