//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1800/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1800<F: Float>(t3726: F, t6375: F, t119: F, t19631: F, t210: F, t12385: F, t6390: F, t16288: F, t1827: F, t1340: F, t19815: F, t12215: F, t1315: F, t1354: F, t16147: F, t16159: F, t16211: F, t16214: F, t16278: F, t16394: F, t19823: F, t19827: F, t19831: F, t19834: F, t19836: F, t19839: F, t3733: F, t5235: F, t5289: F, t5293: F, t5303: F, t559: F) -> (F, F, F, F, F, F) {
    let t19841 = t3726 * t6375;
    let t19843 = t119 * t19631;
    let t19844 = t210 * t19843;
    let t19851 = t12385 * t6390;
    let t19853 = t16288 * t1827;
    let t19855 = t19815 * t1340;
    let t19862 = -t16147 + t16159 - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t16211 + t16214 - t12215 * t19823 / F::cast_from(4.0_f64) + t3733 * t19827 / F::cast_from(8.0_f64) + t3733 * t19831 / F::cast_from(16.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t19834 + t19836 * t559 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t19839 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t19841 - t1315 * t19844 / F::cast_from(48.0_f64) - t16394 * t5293 / F::cast_from(1536.0_f64) + t16394 * t5303 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t19851 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t19853 - t19855 * t1354 / F::cast_from(3072.0_f64) - t16278 * t1827 / F::cast_from(1536.0_f64) - t5235 * t5289 / F::cast_from(1536.0_f64);
    (t19841, t19844, t19851, t19853, t19855, t19862)
}
