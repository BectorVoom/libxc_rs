//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1734/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1734(t2010: f64, t22716: f64, t1998: f64, t3879: f64, t214: f64, t1985: f64, t154: f64, t591: f64) -> (f64, f64, f64, f64, f64) {
    let t22717 = t22716 * t2010;
    let t22718 = 0.63969658155208805863e-1_f64 * t22717;
    let t22719 = t1998 * t3879;
    let t22720 = t214 * t22719;
    let t22721 = t1985 * t22720;
    let t22723 = t591 * t154;
    (t22718, t22719, t22720, t22721, t22723)
}
