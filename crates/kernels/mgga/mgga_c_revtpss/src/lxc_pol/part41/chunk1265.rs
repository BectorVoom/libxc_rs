//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1265/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1265<F: Float>(t1248: F, t6587: F, t1250: F, t3720: F, t17183: F, t5330: F, t17737: F, t5297: F, t3626: F, t1230: F, t6594: F, t1803: F, t5261: F, t12297: F, t12678: F, t16706: F, t17319: F, t17320: F, t17321: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F, F, F, F, F, F, F) {
    let t21298 = t6587 * t1248;
    let t21299 = t21298 * t1250;
    let t21300 = t3720 * t21299;
    let t21306 = t17183 * t5330;
    let t21309 = t17737 * t5297;
    let t21310 = t3626 * t21309;
    let t21313 = t1230 * t6594;
    let t21316 = t5261 * t1803;
    let t21332 = -t12678 + 0.37037037037037037037e-2 * t12297 + 0.74074074074074074074e-2 * t16706 + t17319 - t17320 - t17321 + 0.18518518518518518518e-2 * t20283 + 0.92592592592592592592e-2 * t20295 - 0.33333333333333333333e-1 * t20300 - 0.11111111111111111111e-1 * t20304 - 0.55555555555555555557e-2 * t20285 + 0.50000000000000000001e-1 * t20308 + 0.33333333333333333334e-1 * t20312 - 0.27777777777777777778e-2 * t20287 - 0.55555555555555555555e-2 * t20315 + 0.16666666666666666667e-1 * t20320 + 0.83333333333333333333e-2 * t20290;
    (t21298, t21300, t21306, t21310, t21313, t21316, t21332)
}
