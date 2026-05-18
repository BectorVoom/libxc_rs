//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1129/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1129<F: Float>(t12879: F, t1715: F, t247: F, t1261: F, t12916: F, t5342: F, t5340: F, t127: F, t371: F, t5318: F, t1235: F, t3685: F, t5373: F) -> (F, F, F, F) {
    let t17416 = t247 * t12879 * t1715;
    let t17417 = t1261 * t17416;
    let t17423 = t12916 * t5342;
    let t17425 = F::new(0.57165357490759649296e-3) * t5340 * t17423;
    let t17435 = t371 * t127 * t5318;
    let t17437 = F::new(0.28582678745379824648e-3) * t1235 * t17435;
    let t17444 = t5373 * t3685 / F::new(162.0);
    (t17417, t17425, t17437, t17444)
}
