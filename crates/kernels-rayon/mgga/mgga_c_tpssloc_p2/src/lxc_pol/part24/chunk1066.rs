//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1066/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1066(t12237: f64, t225: f64, t3792: f64, t3850: f64, t6977: f64, t3851: f64, t3901: f64, t1337: f64) -> (f64, f64, f64, f64, f64) {
    let t12238 = t12237 * t225;
    let t12240 = t3792 * t3850;
    let t12241 = t6977 * t12240;
    let t12244 = t3901 * t3851;
    let t12247 = t1337 * t1337;
    let t12248 = 1.0_f64 / t12247;
    (t12238, t12240, t12241, t12244, t12248)
}
