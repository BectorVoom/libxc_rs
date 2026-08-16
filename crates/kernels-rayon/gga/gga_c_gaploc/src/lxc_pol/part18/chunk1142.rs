//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1142/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1142(t2464: f64, t587: f64, t9444: f64, t2487: f64, t9449: f64, t7014: f64, t9368: f64, t2488: f64, t30258: f64, t1391: f64, t9367: f64, t4391: f64, t549: f64, t6510: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30762 = t587 * t2464 * t9444;
    let t30765 = t2487 * t2464 * t9449;
    let t30768 = 0.17041300423964777634e0_f64 * t7014 * t9368;
    let t30770 = t2487 * t2488 * t30258;
    let t30771 = 0.38342925953920749676e0_f64 * t30770;
    let t30773 = t2487 * t1391 * t9367;
    let t30774 = 0.5396411800922179584e0_f64 * t30773;
    let t30778 = 0.23833659967900284446e0_f64 * t4391 * t549 * t6510;
    (t30762, t30765, t30768, t30771, t30774, t30778)
}
