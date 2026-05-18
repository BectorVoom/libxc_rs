//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1373/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1373<F: Float>(t11262: F, t1796: F, t1247: F, t1264: F, t16746: F, t247: F, t12915: F, t5230: F, t5384: F, t1770: F, t3140: F, t3609: F) -> (F, F, F, F, F) {
    let t17361 = t11262 * t1796;
    let t17362 = t1247 * t17361;
    let t17369 = t247 * t1264 * t16746;
    let t17373 = t247 * t12915 * t5230;
    let t17375 = F::new(0.57165357490759649296e-3) * t5384 * t17373;
    let t17376 = t1770 * t3140;
    let t17377 = t17376 * t3609;
    (t17362, t17369, t17375, t17376, t17377)
}
