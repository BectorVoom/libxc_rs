//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 966/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk966(t1011: f64, t3508: f64, t1193: f64, t7372: f64, t7378: f64, t7319: f64, t7327: f64, t1170: f64, t7381: f64, t2121: f64, t210: f64, t7371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24815 = t1011 * t3508;
    let t24826 = t7372 * t1193;
    let t24827 = t24826 * t7378;
    let t24833 = t7319 * t7327;
    let t24844 = t1170 * t7381;
    let t24845 = t2121 * t24844;
    let t24847 = t7371 * t210;
    (t24815, t24826, t24827, t24833, t24845, t24847)
}
