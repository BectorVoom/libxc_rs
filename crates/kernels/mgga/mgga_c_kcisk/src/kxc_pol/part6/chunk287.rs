//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 287/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk287<F: Float>(t503: F, t475: F, t140: F, t299: F, t480: F, t139: F, t201: F) -> (F, F, F, F, F) {
    let t1457 = t503 * t503;
    let t1458 = F::new(1.0) / t1457;
    let t1459 = t475 * t1458;
    let t1469 = F::new(0.26531111111111111111e-1) * t140 * t299 * t480;
    let t1470 = t139 * t201;
    (t1457, t1458, t1459, t1469, t1470)
}
