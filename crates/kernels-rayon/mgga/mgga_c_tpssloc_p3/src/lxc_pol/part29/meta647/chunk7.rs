//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2149/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2149(t25250: f64, t87202: f64, t87712: f64, t25316: f64, t82038: f64, t1888: f64, t232: f64, t47439: f64, t6646: f64, t23110: f64, t23185: f64, t25272: f64) -> (f64, f64, f64, f64) {
    let t87714 = t87712 * t87202 * t25250;
    let t87718 = t82038 * t25316;
    let t87726 = t1888 * t6646 * t47439 * t232;
    let t87729 = t23185 * t23110 * t25272;
    (t87714, t87718, t87726, t87729)
}
