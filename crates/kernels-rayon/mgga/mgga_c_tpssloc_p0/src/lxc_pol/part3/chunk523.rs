//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 523/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk523(t2363: f64, t510: f64, t177: f64, t738: f64, t745: f64) -> (f64, f64, f64) {
    let t2364 = t510 * t2363;
    let t2367 = t738 * t177;
    let t2368 = 1.0_f64 / t2367;
    let t2369 = t745 * t745;
    (t2364, t2368, t2369)
}
