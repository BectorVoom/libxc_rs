//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 670/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk670(t28719: f64, t799: f64, t27: f64, t89: f64, t24981: f64, t684: f64, t7062: f64, t24980: f64, t856: f64, t992: f64, t6334: f64, t25026: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28720 = t799 * t28719;
    let t28722 = t89 * t27 * t28720;
    let t28726 = t24981 * t7062 * t684;
    let t28727 = t24980 * t28726;
    let t28729 = t992 * t856;
    let t28731 = t24981 * t6334 * t28729;
    let t28732 = t24980 * t28731;
    let t28735 = t25026 * t92;
    (t28720, t28722, t28727, t28729, t28732, t28735)
}
