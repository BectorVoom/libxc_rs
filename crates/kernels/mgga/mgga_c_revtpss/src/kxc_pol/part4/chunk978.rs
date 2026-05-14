//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 978/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk978<F: Float>(t1130: F, t3376: F, t1126: F, t3432: F, t3431: F, t418: F, t408: F, t3418: F, t698: F, t240: F, t3698: F, t3361: F, t635: F, t1146: F, t2439: F, t3424: F) -> (F, F, F, F, F, F, F, F) {
    let t12238 = t3376 * t1130;
    let t12243 = t1126 * t3432;
    let t12247 = 1.0 / t3431 / t418;
    let t12248 = t408 * t12247;
    let t12252 = t698 * t3418;
    let t12254 = t240 * t3698;
    let t12256 = 1.0 / t3361 / t635;
    let t12261 = t2439 * t1146;
    let t12263 = t698 * t3424;
    (t12238, t12243, t12248, t12252, t12254, t12256, t12261, t12263)
}
