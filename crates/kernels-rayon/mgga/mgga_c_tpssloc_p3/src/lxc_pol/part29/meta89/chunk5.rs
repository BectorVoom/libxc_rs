//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 587/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk587(t225: f64, t252: f64, t258: f64, t214: f64, t1880: f64, t119: f64, t210: f64) -> (f64, f64, f64, f64) {
    let t1882 = t252 * t225 * t258;
    let t1883 = t214 * t1882;
    let t1884 = t1880 * t1883;
    let t1887 = t210 * t119;
    (t1882, t1883, t1884, t1887)
}
