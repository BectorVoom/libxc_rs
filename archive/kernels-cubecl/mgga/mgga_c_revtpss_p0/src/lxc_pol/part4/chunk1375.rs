//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1375/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1375<F: Float>(t17395: F, t3717: F, t1261: F, t12809: F, t12832: F, t17362: F, t17369: F, t17375: F, t17377: F, t17381: F, t17386: F, t17391: F, t3613: F, t3647: F, t3718: F, t3723: F, t5348: F, t5354: F, t5397: F) -> F {
    let t17396 = t3717 * t17395;
    let t17399 = -F::cast_from(0.47637797908966374413e-4_f64) * t17362 - F::cast_from(0.42874018118069736972e-3_f64) * t12832 * t5354 - F::cast_from(0.28582678745379824648e-3_f64) * t3647 * t5397 - F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t17369 + t17375 - F::cast_from(0.21437009059034868486e-3_f64) * t17377 * t3613 + F::cast_from(0.42874018118069736972e-3_f64) * t12809 * t17381 - t17386 - F::cast_from(0.42874018118069736972e-3_f64) * t12832 * t5348 - F::cast_from(0.42874018118069736972e-3_f64) * t3718 * t17391 + F::cast_from(0.22866142996303859718e-2_f64) * t17396 * t3723;
    t17399
}
