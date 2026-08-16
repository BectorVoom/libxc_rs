//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 966/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk966(t21013: f64, t218: f64, t1528: f64, t17052: f64, t17090: f64, t17092: f64, t21034: f64, t21036: f64, t21038: f64, t21050: f64, t21054: f64, t21061: f64, t259: f64, t4147: f64, t4268: f64, t5637: f64, t5658: f64, t855: f64) -> (f64, f64) {
    let t21064 = t218 * t21013;
    let t21066 = -3.0_f64 * t1528 * t17052 - 3.0_f64 * t1528 * t17090 - 6.0_f64 * t1528 * t17092 - t21034 * t855 + t21036 * t259 + 3.0_f64 * t21038 * t259 - 6.0_f64 * t21050 * t855 + 6.0_f64 * t21054 * t855 + 3.0_f64 * t21061 * t259 + t21064 * t259 + 6.0_f64 * t4147 * t5637 - 3.0_f64 * t4147 * t5658 + 6.0_f64 * t4268 * t5637 - 3.0_f64 * t4268 * t5658;
    (t21064, t21066)
}
