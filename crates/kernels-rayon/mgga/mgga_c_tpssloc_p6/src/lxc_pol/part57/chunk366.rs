//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 366/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk366(t2471: f64, t731: f64, t723: f64, t159: f64, t167: f64, t2461: f64, t676: f64, t682: f64, t268: f64, t703: f64) -> (f64, f64, f64, f64) {
    let t2472 = t2471 * t731;
    let t2475 = t723 * t723;
    let t2476 = 1.0_f64 / t2475;
    let t2477 = t159 * t2476;
    let t2478 = t167 * t167;
    let t2479 = 1.0_f64 / t2478;
    let t2480 = t2461 * t2479;
    let t2483 = t676 * t682;
    let t2486 = 0.35616666666666666666e-1_f64 * t268 * t2483 * t703;
    (t2472, t2477, t2480, t2486)
}
