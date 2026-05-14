//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1040/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1040<F: Float>(t11262: F, t1796: F, t1247: F, t12915: F, t247: F, t5230: F, t5384: F, t12772: F, t5406: F, t3625: F, t1802: F, t474: F, t3089: F, t3717: F, t1284: F, t5219: F) -> (F, F, F, F, F, F, F) {
    let t17361 = t11262 * t1796;
    let t17362 = t1247 * t17361;
    let t17373 = t247 * t12915 * t5230;
    let t17375 = 0.57165357490759649296e-3 * t5384 * t17373;
    let t17384 = t12772 * t5406;
    let t17386 = 0.19055119163586549765e-3 * t3625 * t17384;
    let t17394 = t474 * t1802;
    let t17395 = t17394 * t3089;
    let t17396 = t3717 * t17395;
    let t17400 = t5219 * t1284;
    (t17362, t17375, t17386, t17394, t17395, t17396, t17400)
}
