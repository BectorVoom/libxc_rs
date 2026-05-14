//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 932/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk932<F: Float>(t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t12329: F, t12332: F, t12295: F, t12351: F) -> (F, F, F) {
    let t12531 = 0.5519e-1 * t12252 + 0.36793333333333333333e-1 * t12259 + 0.27595e0 * t12261 - 0.16557e0 * t12263 - 0.33114e0 * t12265 - 0.16557e0 * t12271 + 0.49671e0 * t12275 + 0.82785e-1 * t12279 - 0.82785e-1 * t12284 + 0.49671e0 * t12289 - 0.60384999999999999999e0 * t12292 + 0.258925e1 * t12323 + 0.19419375e1 * t12329 - 0.412621875e-1 * t12332;
    let t12542 = 0.93932222222222222223e0 * t12295;
    let t12543 = 0.36793333333333333333e0 * t12351;
    (t12531, t12542, t12543)
}
