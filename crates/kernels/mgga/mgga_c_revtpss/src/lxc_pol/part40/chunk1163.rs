//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1163/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1163<F: Float>(t15220: F, t923: F, t916: F, t11134: F, t11136: F, t11138: F, t11140: F, t11339: F, t11366: F, t11368: F, t11479: F, t11480: F, t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15125: F, t15128: F, t15132: F, t15175: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15192: F, t15195: F, t15198: F, t15200: F) -> (F, F, F) {
    let t15221 = t923 * t15220;
    let t15230 = t916 * t15220;
    let t15232 = -t11479 - t11480 + 0.16504875e0 * t15221 + 0.18396666666666666667e-1 * t11339 - 0.20128333333333333334e0 * t11138 - 0.26837777777777777778e0 * t11134 + 0.10064166666666666667e0 * t11140 + 0.67094444444444444447e-1 * t11136 - 0.18396666666666666667e0 * t11366 + 0.5519e-1 * t11368 + 0.258925e1 * t15230;
    let t15234 = 0.19419375e1 * t15108 - 0.412621875e-1 * t15111 - 0.258925e1 * t15114 - 0.1294625e1 * t15116 + 0.16504875e0 * t15119 + 0.82524375e-1 * t15121 - 0.91983333333333333334e-1 * t15123 - 0.40256666666666666667e0 * t15125 + t15128 - 0.40256666666666666666e0 * t15132 + t15175 - 0.27595e-1 * t15178 - 0.36793333333333333333e-1 * t15181 + 0.33114e0 * t15184 + 0.16557e0 * t15187 - 0.13418888888888888889e0 * t15189 + t15192 - 0.301925e0 * t15195 + t15198 - 0.82785e-1 * t15200 - 0.11038e0 * t11326 + t15232;
    (t15221, t15230, t15234)
}
