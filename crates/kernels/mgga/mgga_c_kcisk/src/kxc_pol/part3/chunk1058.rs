//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1058/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1058<F: Float>(t15744: F, t15759: F, t10339: F, t10342: F, t10351: F, t1140: F, t1147: F, t12815: F, t15452: F, t15463: F, t15473: F, t15711: F, t15713: F, t15716: F, t15723: F, t15724: F, t15727: F, t289: F, t3437: F, t3442: F, t3443: F, t3460: F) -> F {
    let t15760 = t15744 + t15759;
    let t15762 = -t1140 * t15760 - F::new(3.0) * t1147 * t15713 + t15711 * t289 + F::new(6.0) * t15716 * t3443 - F::new(6.0) * t15723 * t15724 + F::new(6.0) * t15727 * t3442 - F::new(3.0) * t3437 * t3460 + t10339 - t10342 - t10351 + t12815 - t15452 + t15463 + t15473;
    t15762
}
