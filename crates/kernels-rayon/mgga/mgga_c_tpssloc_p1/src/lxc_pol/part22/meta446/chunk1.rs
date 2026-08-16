//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1800/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1800(t3726: f64, t6375: f64, t119: f64, t19631: f64, t210: f64, t12385: f64, t6390: f64, t16288: f64, t1827: f64, t1340: f64, t19815: f64, t12215: f64, t1315: f64, t1354: f64, t16147: f64, t16159: f64, t16211: f64, t16214: f64, t16278: f64, t16394: f64, t19823: f64, t19827: f64, t19831: f64, t19834: f64, t19836: f64, t19839: f64, t3733: f64, t5235: f64, t5289: f64, t5293: f64, t5303: f64, t559: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19841 = t3726 * t6375;
    let t19843 = t119 * t19631;
    let t19844 = t210 * t19843;
    let t19851 = t12385 * t6390;
    let t19853 = t16288 * t1827;
    let t19855 = t19815 * t1340;
    let t19862 = -t16147 + t16159 - 119.0_f64 / 6912.0_f64 * t16211 + t16214 - t12215 * t19823 / 4.0_f64 + t3733 * t19827 / 8.0_f64 + t3733 * t19831 / 16.0_f64 - 7.0_f64 / 4608.0_f64 * t19834 + t19836 * t559 / 3072.0_f64 - 7.0_f64 / 48.0_f64 * t19839 + 7.0_f64 / 144.0_f64 * t19841 - t1315 * t19844 / 48.0_f64 - t16394 * t5293 / 1536.0_f64 + t16394 * t5303 / 384.0_f64 - 7.0_f64 / 2304.0_f64 * t19851 + 7.0_f64 / 2304.0_f64 * t19853 - t19855 * t1354 / 3072.0_f64 - t16278 * t1827 / 1536.0_f64 - t5235 * t5289 / 1536.0_f64;
    (t19841, t19844, t19851, t19853, t19855, t19862)
}
