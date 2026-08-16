//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2066/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2066(t214: f64, t5318: f64, t26378: f64, t6914: f64, t1372: f64, t1799: f64, t26411: f64, t22704: f64, t22705: f64, t5345: f64, t22690: f64, t552: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90739 = t214 * t5318;
    let t90749 = t6914 * t26378;
    let t90750 = 0.76763589786250567036e-1_f64 * t90749;
    let t90754 = t1372 * t1799;
    let t90759 = t6914 * t26411;
    let t90760 = 0.38381794893125283518e-1_f64 * t90759;
    let t90781 = t22704 * t22705 * t5345;
    let t90782 = 0.82246703342411321824e-2_f64 * t90781;
    let t90787 = t22690 * t552;
    (t90739, t90750, t90754, t90760, t90782, t90787)
}
