//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1340/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1340(t1210: f64, t29647: f64, t86264: f64, t8040: f64, t95332: f64, t29561: f64, t6739: f64, t7325: f64, t27628: f64, t95648: f64, t104118: f64, t24682: f64, t460: f64) -> (f64, f64, f64, f64, f64) {
    let t104184 = t86264 * t1210 * t29647;
    let t104187 = t95332 * t8040;
    let t104190 = t29561 * t6739 * t7325;
    let t104231 = t95648 * t27628;
    let t104235 = t24682 * t104118 * t460;
    (t104184, t104187, t104190, t104231, t104235)
}
