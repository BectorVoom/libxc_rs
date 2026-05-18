//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 308/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk308<F: Float>(t1455: F, t504: F, t503: F, t475: F, t1216: F, t196: F, t140: F, t299: F, t480: F, t139: F, t201: F) -> (F, F, F, F, F, F, F) {
    let t1456 = t1455 * t504;
    let t1457 = t503 * t503;
    let t1458 = F::new(1.0) / t1457;
    let t1459 = t475 * t1458;
    let t1460 = t1216 * t196;
    let t1469 = F::new(0.26531111111111111111e-1) * t140 * t299 * t480;
    let t1470 = t139 * t201;
    (t1456, t1457, t1458, t1459, t1460, t1469, t1470)
}
