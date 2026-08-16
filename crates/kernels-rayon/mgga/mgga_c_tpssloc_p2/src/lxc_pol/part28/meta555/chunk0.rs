//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1826/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1826(t81146: f64, t81153: f64, t225: f64, t24162: f64, t81317: f64, t24064: f64, t81398: f64, t2056: f64, t40772: f64, t24334: f64, t2752: f64, t193: f64, t201: f64, t7109: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84595 = 0.27415567780803773942e-2_f64 * t81146;
    let t84597 = 0.19739208802178717238e0_f64 * t81153;
    let t84655 = t24162 * t225;
    let t84659 = 0.55440370401180965083e0_f64 * t81317;
    let t84700 = t24064 * t225;
    let t84705 = 0.27415567780803773942e-2_f64 * t81398;
    let t84766 = t2056 * t40772;
    let t84791 = t24334 * t2752;
    let t84797 = t193 * t201 * t7109;
    (t84595, t84597, t84655, t84659, t84700, t84705, t84766, t84791, t84797)
}
