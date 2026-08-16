//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 485/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk485(t203: f64, t2465: f64, t2464: f64, t587: f64, t447: f64, t487: f64, t2365: f64, t1416: f64, t1421: f64, t901: f64, t1433: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2466 = t2465 * t203;
    let t2467 = t2464 * t2466;
    let t2468 = t587 * t2467;
    let t2470 = t487 * t447;
    let t2471 = t2365 * t2470;
    let t2472 = t1416 * t2471;
    let t2474 = t1421 * t901;
    let t2476 = t1433 * t586;
    (t2466, t2467, t2468, t2470, t2471, t2472, t2474, t2476)
}
