//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 870/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk870(t4295: f64, t5617: f64, t16891: f64, t2645: f64, t5591: f64, t232: f64, t5544: f64, t4181: f64, t1510: f64, t4180: f64, t20756: f64, t820: f64, t9607: f64) -> (f64, f64, f64, f64, f64) {
    let t20876 = t4295 * t5617;
    let t20882 = t2645 * t16891 * t5591;
    let t20885 = t232 * t5544;
    let t20887 = t2645 * t4181 * t20885;
    let t20891 = t4180 * t16891 * t1510;
    let t20896 = t9607 * t820 * t20756;
    (t20876, t20882, t20887, t20891, t20896)
}
