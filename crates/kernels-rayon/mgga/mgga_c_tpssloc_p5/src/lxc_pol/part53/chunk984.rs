//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 984/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk984(t114797: f64, t1484: f64, t22986: f64, t23270: f64, t33448: f64, t81591: f64, t1888: f64, t33457: f64, t82159: f64, t1880: f64, t214: f64, t225: f64, t258: f64, t26653: f64) -> (f64, f64, f64, f64) {
    let t121367 = t22986 * t23270 * t114797 * t1484;
    let t121371 = t81591 * t33448;
    let t121382 = t1888 * t82159 * t33457;
    let t121391 = t1880 * t214 * t26653 * t225 * t258;
    (t121367, t121371, t121382, t121391)
}
