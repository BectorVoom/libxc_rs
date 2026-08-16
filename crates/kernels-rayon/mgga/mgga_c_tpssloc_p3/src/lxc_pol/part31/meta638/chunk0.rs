//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1906/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1906(t1388: f64, t6324: f64, t24994: f64, t7684: f64, t1307: f64, t28830: f64, t19534: f64, t89: f64, t16944: f64, t25014: f64, t25365: f64, t86721: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97875 = t6324 * t1388;
    let t97890 = t7684 * t24994;
    let t97894 = t6324 * t1307;
    let t97902 = t28830 * t1307;
    let t97911 = t28830 * t1388;
    let t97933 = t89 * t19534;
    let t97950 = t25014 * t16944;
    let t97953 = t86721 * t25365;
    (t97875, t97890, t97894, t97902, t97911, t97933, t97950, t97953)
}
