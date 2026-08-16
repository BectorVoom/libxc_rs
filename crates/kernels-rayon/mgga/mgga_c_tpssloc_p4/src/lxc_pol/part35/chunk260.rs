//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 260/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk260(t134: f64, t340: f64, t344: f64, t221: f64, t339: f64, t209: f64, t338: f64, t39: f64) -> (f64, f64, f64, f64) {
    let t967 = t134 * t340;
    let t968 = t967 * t344;
    let t969 = t221 * t968;
    let t971 = 0.27777777777777777777e-3_f64 * t339 * t969;
    let t972 = t338 * t209;
    let t973 = t39 * t972;
    (t967, t971, t972, t973)
}
