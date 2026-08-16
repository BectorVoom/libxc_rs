//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1850/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1850(t87931: f64, t10143: f64, t7844: f64, t27143: f64, t532: f64, t90459: f64, t90468: f64, t90470: f64, t90472: f64, t225: f64, t27137: f64, t27059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92976 = 0.15352717957250113407e0_f64 * t87931;
    let t93000 = t7844 * t10143;
    let t93286 = t532 * t27143;
    let t93306 = 0.76763589786250567036e-1_f64 * t90459;
    let t93309 = 0.15352717957250113407e0_f64 * t90468;
    let t93310 = 0.15352717957250113407e0_f64 * t90470;
    let t93311 = 0.15352717957250113407e0_f64 * t90472;
    let t93313 = t27137 * t225;
    let t93316 = t27059 * t225;
    (t92976, t93000, t93286, t93306, t93309, t93310, t93311, t93313, t93316)
}
