//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2093/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2093(t25038: f64, t25040: f64, t82159: f64, t23030: f64, t25035: f64, t23228: f64, t7479: f64, t81573: f64, t22986: f64, t23270: f64, t25191: f64, t2742: f64) -> (f64, f64, f64, f64) {
    let t86909 = t25038 * t82159 * t25040;
    let t86911 = t23030 * t25035;
    let t86916 = t81573 * t23228 * t7479;
    let t86923 = t22986 * t23270 * t25191 * t2742;
    (t86909, t86911, t86916, t86923)
}
