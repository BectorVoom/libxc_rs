//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1277/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1277<F: Float>(t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15125: F, t15128: F, t15132: F, t15175: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15192: F, t15195: F, t15198: F, t15200: F, t15232: F) -> F {
    let t15234 = F::cast_from(0.19419375e1_f64) * t15108 - F::cast_from(0.412621875e-1_f64) * t15111 - F::cast_from(0.258925e1_f64) * t15114 - F::cast_from(0.1294625e1_f64) * t15116 + F::cast_from(0.16504875e0_f64) * t15119 + F::cast_from(0.82524375e-1_f64) * t15121 - F::cast_from(0.91983333333333333334e-1_f64) * t15123 - F::cast_from(0.40256666666666666667e0_f64) * t15125 + t15128 - F::cast_from(0.40256666666666666666e0_f64) * t15132 + t15175 - F::cast_from(0.27595e-1_f64) * t15178 - F::cast_from(0.36793333333333333333e-1_f64) * t15181 + F::cast_from(0.33114e0_f64) * t15184 + F::cast_from(0.16557e0_f64) * t15187 - F::cast_from(0.13418888888888888889e0_f64) * t15189 + t15192 - F::cast_from(0.301925e0_f64) * t15195 + t15198 - F::cast_from(0.82785e-1_f64) * t15200 - F::cast_from(0.11038e0_f64) * t11326 + t15232;
    t15234
}
