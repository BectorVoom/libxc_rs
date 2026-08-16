//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1401/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1401(t225: f64, t23410: f64, t6692: f64, t82632: f64, t6707: f64, t82573: f64, t6695: f64, t3166: f64, t6703: f64, t1049: f64, t6733: f64, t10160: f64, t10165: f64, t10181: f64, t10316: f64, t10327: f64, t1052: f64, t1055: f64, t1066: f64, t1922: f64, t23310: f64, t23327: f64, t23332: f64, t23346: f64, t23722: f64, t25757: f64, t25758: f64, t3169: f64, t3175: f64, t6687: f64, t6689: f64, t6690: f64, t6691: f64, t6706: f64, t6776: f64, t6815: f64, t82499: f64, t82502: f64, t82561: f64, t82603: f64, t82660: f64, t82705: f64, t82749: f64, t82795: f64, t82834: f64, t83270: f64) -> f64 {
    let t83276 = t23410 * t225;
    let t83281 = t82632 * t6692;
    let t83285 = t82573 * t6707;
    let t83287 = t82573 * t6695;
    let t83296 = t6703 * t3166;
    let t83303 = t6733 * t1049;
    let t83307 = 0.16449340668482264365e-1_f64 * t6687 * t6689 * t6690 * t10316 + 0.13159472534785811492e0_f64 * t23346 * t23310 - 3.0_f64 * t82499 * t1066 + 0.16449340668482264365e-1_f64 * t23327 * t82502 * t23332 - t1052 * t1055 * (t82561 + t82603 + t82660 + t82705 + t82749 + t82795 + t82834 + t83270) - 6.0_f64 * t83276 * t1066 + 12.0_f64 * t10160 * t6776 - 0.18277045187202515961e-2_f64 * t83281 - 3.0_f64 * t3169 * t23722 + 0.43864908449286038307e-1_f64 * t83285 + 0.43864908449286038307e-1_f64 * t83287 - 0.82246703342411321825e-2_f64 * t6687 * t10327 * t1922 - 18.0_f64 * t1052 * t10165 * t6815 * t3175 - 0.24674011002723396548e-1_f64 * t6687 * t83296 * t6706 - 18.0_f64 * t25757 * t25758 * t10181 - 0.16449340668482264365e-1_f64 * t23327 * t83303 * t6691;
    t83307
}
