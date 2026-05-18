//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1128/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1128<F: Float>(t12772: F, t5406: F, t3625: F, t1802: F, t474: F, t3089: F, t3717: F, t1284: F, t5219: F, t3624: F, t1230: F, t5390: F) -> (F, F, F, F, F, F) {
    let t17384 = t12772 * t5406;
    let t17386 = F::new(0.19055119163586549765e-3) * t3625 * t17384;
    let t17394 = t474 * t1802;
    let t17395 = t17394 * t3089;
    let t17396 = t3717 * t17395;
    let t17400 = t5219 * t1284;
    let t17401 = t17400 * t3624;
    let t17412 = t1230 * t5390;
    (t17386, t17394, t17395, t17396, t17401, t17412)
}
