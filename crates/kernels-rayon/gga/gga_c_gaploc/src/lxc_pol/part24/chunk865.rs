//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 865/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk865(t550: f64, t8528: f64, t549: f64, t1402: f64, t2954: f64, t2963: f64, t590: f64, t701: f64, t1457: f64, t8512: f64, t3039: f64, t783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8529 = t550 * t8528;
    let t8530 = t549 * t8529;
    let t8535 = t1402 * t2954;
    let t8540 = t2963 * t590;
    let t8549 = t8528 * t701;
    let t8550 = t1457 * t8549;
    let t8553 = t1457 * t8512;
    let t8556 = t3039 * t783;
    (t8529, t8530, t8535, t8540, t8549, t8550, t8553, t8556)
}
