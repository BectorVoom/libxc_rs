//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 352/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk352<F: Float>(t1118: F, t1143: F, t1124: F, t1135: F, t1140: F, t1147: F) -> (F, F, F) {
    let t1163 = 0.516475e0 * t1118;
    let t1166 = 0.104195e0 * t1143;
    let t1168 = 0.3529725e1 * t1135 - t1163 + 0.516475e0 * t1124 + 0.6311625e0 * t1140 - t1166 + 0.104195e0 * t1147;
    (t1163, t1166, t1168)
}
