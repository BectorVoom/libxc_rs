//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1231/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1231<F: Float>(t17384: F, t3625: F, t1248: F, t5245: F, t1250: F, t3720: F, t1802: F, t474: F, t3089: F, t3717: F, t1261: F, t12809: F, t12832: F, t17362: F, t17369: F, t17375: F, t17377: F, t17381: F, t3613: F, t3647: F, t3718: F, t3723: F, t5348: F, t5354: F, t5397: F) -> (F, F, F) {
    let t17386 = 0.19055119163586549765e-3 * t3625 * t17384;
    let t17389 = t5245 * t1248;
    let t17390 = t17389 * t1250;
    let t17391 = t3720 * t17390;
    let t17394 = t474 * t1802;
    let t17395 = t17394 * t3089;
    let t17396 = t3717 * t17395;
    let t17399 = -0.47637797908966374413e-4 * t17362 - 0.42874018118069736972e-3 * t12832 * t5354 - 0.28582678745379824648e-3 * t3647 * t5397 - 0.14291339372689912324e-3 * t1261 * t17369 + t17375 - 0.21437009059034868486e-3 * t17377 * t3613 + 0.42874018118069736972e-3 * t12809 * t17381 - t17386 - 0.42874018118069736972e-3 * t12832 * t5348 - 0.42874018118069736972e-3 * t3718 * t17391 + 0.22866142996303859718e-2 * t17396 * t3723;
    (t17389, t17395, t17399)
}
