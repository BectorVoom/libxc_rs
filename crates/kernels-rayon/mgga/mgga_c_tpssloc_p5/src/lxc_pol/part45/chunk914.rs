//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 914/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk914(t2085: f64, t3850: f64, t225: f64, t24162: f64, t24064: f64, t2056: f64, t40772: f64, t24334: f64, t2752: f64, t193: f64, t201: f64, t7109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t84441 = t2085 * t3850;
    let t84655 = t24162 * t225;
    let t84700 = t24064 * t225;
    let t84766 = t2056 * t40772;
    let t84791 = t24334 * t2752;
    let t84797 = t193 * t201 * t7109;
    (t84441, t84655, t84700, t84766, t84791, t84797)
}
