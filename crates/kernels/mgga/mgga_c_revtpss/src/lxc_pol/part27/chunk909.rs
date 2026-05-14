//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 909/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk909<F: Float>(t1151: F, t12238: F, t3379: F, t3428: F, t1126: F, t3432: F, t3436: F, t3431: F, t418: F, t408: F, t12228: F, t3435: F, t3418: F, t698: F, t240: F, t3698: F) -> (F, F, F, F, F, F) {
    let t12240 = 3.0 * t12238 * t1151;
    let t12242 = 3.0 * t3379 * t3428;
    let t12243 = t1126 * t3432;
    let t12245 = 0.48245938496077605201e2 * t12243 * t3436;
    let t12247 = 1.0 / t3431 / t418;
    let t12248 = t408 * t12247;
    let t12249 = t12228 * t3435;
    let t12251 = 0.96491876992155210402e2 * t12248 * t12249;
    let t12252 = t698 * t3418;
    let t12254 = t240 * t3698;
    (t12240, t12242, t12245, t12251, t12252, t12254)
}
