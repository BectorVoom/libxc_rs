//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1193/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1193(t31270: f64, t31272: f64, t31274: f64, t31277: f64, t31279: f64, t31282: f64, t31284: f64, t31287: f64, t31923: f64, t31937: f64, t31940: f64, t31942: f64, t31944: f64, t577: f64, t671: f64, t8508: f64) -> f64 {
    let t31949 = 0.45e1_f64 * t31923 * t577 + 0.135e2_f64 * t31937 * t671 + 0.135e2_f64 * t31940 + 27.0_f64 * t31942 + 0.135e2_f64 * t31944 + 0.135e2_f64 * t31270 + 27.0_f64 * t31272 + 0.135e2_f64 * t31274 + t31277 + t31279 + t31282 + t31284 + t31287 + t8508;
    t31949
}
