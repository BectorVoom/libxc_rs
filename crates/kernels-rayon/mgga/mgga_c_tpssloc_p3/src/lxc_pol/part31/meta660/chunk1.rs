//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1946/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1946(t16949: f64, t221: f64, t25154: f64, t25119: f64, t841: f64, t25038: f64, t25248: f64, t776: f64, t98422: f64, t23110: f64, t23185: f64, t28321: f64) -> (f64, f64, f64, f64) {
    let t98868 = t25154 * t221 * t16949;
    let t98871 = t25119 * t841 * t16949;
    let t98881 = t25038 * t25248 * t98422 * t776;
    let t98884 = t23185 * t23110 * t28321;
    (t98868, t98871, t98881, t98884)
}
