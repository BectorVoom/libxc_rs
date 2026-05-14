//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 934/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk934<F: Float>(t12694: F, t12701: F, t12703: F, t12706: F, t12708: F, t12710: F, t12714: F, t12717: F, t12771: F, t12774: F, t12776: F, t12779: F, t12782: F, t12811: F, t15744: F, t10339: F, t10342: F, t10351: F, t1140: F, t1147: F, t12815: F, t15452: F, t15463: F, t15473: F, t15711: F, t15713: F, t15716: F, t15723: F, t15724: F, t15727: F, t289: F, t3437: F, t3442: F, t3443: F, t3460: F) -> (F,) {
    let t15759 = 0.1125e1 * t12694 + 0.2428125e0 * t12701 - 0.3375e1 * t12703 + 0.12140625e0 * t12706 - 0.5625e0 * t12708 - 0.97125e0 * t12710 - 0.1125e1 * t12714 + 0.97125e0 * t12717 + 0.4046875e-1 * t12771 - 0.485625e0 * t12774 + 0.12140625e0 * t12776 - 0.1875e0 * t12779 + 0.1125e1 * t12782 - 0.4046875e-1 * t12811;
    let t15760 = t15744 + t15759;
    let t15762 = -t1140 * t15760 - 3.0 * t1147 * t15713 + t15711 * t289 + 6.0 * t15716 * t3443 - 6.0 * t15723 * t15724 + 6.0 * t15727 * t3442 - 3.0 * t3437 * t3460 + t10339 - t10342 - t10351 + t12815 - t15452 + t15463 + t15473;
    (t15762,)
}
