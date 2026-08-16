//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2241/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2241(t17395: f64, t3717: f64, t1261: f64, t12809: f64, t12832: f64, t17362: f64, t17369: f64, t17375: f64, t17377: f64, t17381: f64, t17386: f64, t17391: f64, t3613: f64, t3647: f64, t3718: f64, t3723: f64, t5348: f64, t5354: f64, t5397: f64) -> (f64, f64) {
    let t17396 = t3717 * t17395;
    let t17399 = -0.47637797908966374413e-4_f64 * t17362 - 0.42874018118069736972e-3_f64 * t12832 * t5354 - 0.28582678745379824648e-3_f64 * t3647 * t5397 - 0.14291339372689912324e-3_f64 * t1261 * t17369 + t17375 - 0.21437009059034868486e-3_f64 * t17377 * t3613 + 0.42874018118069736972e-3_f64 * t12809 * t17381 - t17386 - 0.42874018118069736972e-3_f64 * t12832 * t5348 - 0.42874018118069736972e-3_f64 * t3718 * t17391 + 0.22866142996303859718e-2_f64 * t17396 * t3723;
    (t17396, t17399)
}
