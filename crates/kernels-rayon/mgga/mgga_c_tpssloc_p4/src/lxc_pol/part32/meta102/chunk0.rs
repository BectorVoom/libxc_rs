//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 649/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk649(t94: f64, t102: f64, t177: f64, t738: f64, t745: f64) -> (f64, f64, f64, f64) {
    let t2341 = 1.0_f64 / t94;
    let t2349 = 1.0_f64 / t102;
    let t2367 = t738 * t177;
    let t2368 = 1.0_f64 / t2367;
    let t2369 = t745 * t745;
    (t2341, t2349, t2368, t2369)
}
