//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1180/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1180<F: Float>(t23336: F, t27261: F, t23323: F, t25270: F, t27221: F, t76613: F, t23267: F, t7025: F, t23263: F, t92981: F, t23281: F, t7045: F) -> (F, F, F, F, F, F) {
    let t113186 = t27261 * t23336;
    let t113188 = t25270 * t23323;
    let t113214 = t27221 * t76613;
    let t113217 = t7025 * t23267;
    let t113222 = t92981 * t23263;
    let t113226 = t7045 * t23281;
    (t113186, t113188, t113214, t113217, t113222, t113226)
}
