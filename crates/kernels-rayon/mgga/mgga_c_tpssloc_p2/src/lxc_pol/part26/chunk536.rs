//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 536/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk536(t912: f64, t913: f64, t2792: f64, t273: f64, t276: f64, t896: f64, t2764: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2793 = t912 * t912;
    let t2794 = t2793 * t913;
    let t2796 = 2.0_f64 * t2792 * t2794;
    let t2798 = 1.0_f64 / t276 / t273;
    let t2799 = t896 * t896;
    let t2800 = t2798 * t2799;
    let t2802 = 4.0_f64 / 9.0_f64 * t2764;
    let t2807 = t2802 + 2.0_f64 / 9.0_f64 * t2766 - 2.0_f64 / 9.0_f64 * t2773 + 2.0_f64 / 3.0_f64 * t2778 - t2782 / 3.0_f64;
    let t2808 = t894 * t2807;
    (t2793, t2794, t2796, t2798, t2799, t2800, t2807, t2808)
}
