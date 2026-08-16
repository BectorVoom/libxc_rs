//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 111/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk111(t367: f64, t372: f64, t374: f64, t365: f64, t6: f64, t8: f64, t103: f64, t61: f64) -> (f64, f64, f64, f64) {
    let t375 = t367 * t372 * t374;
    let t377 = 0.58482233974552040708e0_f64 * t365 * t375;
    let t378 = t6 * t8;
    let t380 = t61 * t378 * t103;
    (t375, t377, t378, t380)
}
