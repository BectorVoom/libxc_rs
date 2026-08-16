//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 144/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk144(t481: f64, t6: f64, t134: f64, t128: f64, t137: f64, t139: f64, t124: f64, t193: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t482 = t6 * t481;
    let t483 = t482 * t134;
    let t486 = t482 * t128;
    let t487 = t137 * t139;
    let t488 = t487 * t124;
    let t491 = 1.0_f64 / t193;
    let t492 = t5 * t491;
    (t482, t483, t486, t487, t488, t491, t492)
}
