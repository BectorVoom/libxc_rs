//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1231/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1231(t11326: f64, t15108: f64, t15111: f64, t15114: f64, t15116: f64, t15119: f64, t15121: f64, t15123: f64, t15125: f64, t15128: f64, t15132: f64, t15175: f64, t15178: f64, t15181: f64, t15184: f64, t15187: f64, t15189: f64, t15192: f64, t15195: f64, t15198: f64, t15200: f64, t15232: f64) -> f64 {
    let t15234 = 0.19419375e1_f64 * t15108 - 0.412621875e-1_f64 * t15111 - 0.258925e1_f64 * t15114 - 0.1294625e1_f64 * t15116 + 0.16504875e0_f64 * t15119 + 0.82524375e-1_f64 * t15121 - 0.91983333333333333334e-1_f64 * t15123 - 0.40256666666666666667e0_f64 * t15125 + t15128 - 0.40256666666666666666e0_f64 * t15132 + t15175 - 0.27595e-1_f64 * t15178 - 0.36793333333333333333e-1_f64 * t15181 + 0.33114e0_f64 * t15184 + 0.16557e0_f64 * t15187 - 0.13418888888888888889e0_f64 * t15189 + t15192 - 0.301925e0_f64 * t15195 + t15198 - 0.82785e-1_f64 * t15200 - 0.11038e0_f64 * t11326 + t15232;
    t15234
}
