//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1046/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1046(t2206: f64, t932: f64, t6851: f64, t761: f64, t147: f64, t19: f64, t2254: f64, t3296: f64, t2153: f64, t5692: f64, t8: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24352 = t932 * t2206;
    let t24398 = t761 * t6851;
    let t24499 = t3296 * t2254 * t19 * t147;
    let t24625 = t2153 * t2206;
    let t24759 = 1.0_f64 / t8 / t5692;
    let t24760 = t5 * t24759;
    (t24352, t24398, t24499, t24625, t24759, t24760)
}
