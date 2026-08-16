//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1120/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1120(t1988: f64, t22716: f64, t22724: f64, t6898: f64, t225: f64, t3886: f64, t1914: f64, t193: f64, t201: f64) -> (f64, f64, f64, f64) {
    let t22923 = t22716 * t1988;
    let t22924 = 0.63969658155208805863e-1_f64 * t22923;
    let t22925 = t22724 * t6898;
    let t22926 = 0.26044789391763585244e-1_f64 * t22925;
    let t22933 = t225 * t3886;
    let t22959 = t193 * t201 * t1914;
    (t22924, t22926, t22933, t22959)
}
