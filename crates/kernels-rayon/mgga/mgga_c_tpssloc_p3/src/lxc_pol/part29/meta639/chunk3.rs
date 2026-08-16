//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2103/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2103(t1888: f64, t232: f64, t47448: f64, t6646: f64, t23110: f64, t23185: f64, t25241: f64, t25038: f64, t25248: f64, t25249: f64, t2553: f64, t1519: f64, t2631: f64) -> (f64, f64, f64, f64) {
    let t87097 = t1888 * t6646 * t47448 * t232;
    let t87100 = t23185 * t23110 * t25241;
    let t87101 = 0.82246703342411321824e-2_f64 * t87100;
    let t87104 = t25038 * t25248 * t25249 * t2553;
    let t87106 = t1519 * t2631;
    (t87097, t87101, t87104, t87106)
}
