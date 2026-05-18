//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1086/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1086<F: Float>(t10726: F, t14868: F, t2661: F, t10868: F, t241: F, t820: F, t10811: F, t4452: F, t2719: F, t844: F, t4368: F, t2482: F, t814: F) -> (F, F, F, F, F) {
    let t14869 = t10726 * t14868;
    let t14871 = F::new(0.28582678745379824648e-4) * t2661 * t14869;
    let t14894 = t820 * t10868 * t241;
    let t14907 = t10811 * t4452;
    let t14923 = t820 * t2719 * t844;
    let t14925 = F::new(0.40015750243531754508e-2) * t14923 * t4368;
    let t14931 = t2482 * t2719 * t814;
    (t14871, t14894, t14907, t14925, t14931)
}
