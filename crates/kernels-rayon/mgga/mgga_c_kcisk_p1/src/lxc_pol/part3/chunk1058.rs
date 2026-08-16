//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1058/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1058(t15744: f64, t15759: f64, t10339: f64, t10342: f64, t10351: f64, t1140: f64, t1147: f64, t12815: f64, t15452: f64, t15463: f64, t15473: f64, t15711: f64, t15713: f64, t15716: f64, t15723: f64, t15724: f64, t15727: f64, t289: f64, t3437: f64, t3442: f64, t3443: f64, t3460: f64) -> f64 {
    let t15760 = t15744 + t15759;
    let t15762 = -t1140 * t15760 - 3.0_f64 * t1147 * t15713 + t15711 * t289 + 6.0_f64 * t15716 * t3443 - 6.0_f64 * t15723 * t15724 + 6.0_f64 * t15727 * t3442 - 3.0_f64 * t3437 * t3460 + t10339 - t10342 - t10351 + t12815 - t15452 + t15463 + t15473;
    t15762
}
