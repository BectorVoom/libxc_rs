//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 775/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk775<F: Float>(t3128: F, t60: F, t170: F, t596: F, t7647: F, t7650: F, t7653: F, t7656: F, t7659: F, t7661: F, t7662: F, t7664: F, t7667: F, t7669: F, t7671: F, t5248: F, t5253: F, t5258: F, t5263: F, t5274: F, t5278: F, t5282: F, t5288: F, t5295: F, t5298: F, t5302: F, t5303: F) -> (F, F) {
    let t8892 = t60 * t3128;
    let t8893 = t8892 * t170;
    let t8896 = 0.38527786510141256861e1 * t7647 + 0.3429168e0 * t7650 + t7653 + t7656 + t7659 + t7661 - 0.2077903092681775651e3 * t7662 + 0.70178683471615754484e1 * t7664 - 0.67745118933333333332e-2 * t7667 + 0.14458108400402319789e-1 * t7669 - 40.0 * t7671 - 0.675260332e-1 * t596 * t8893;
    let t8901 = t5248 - 0.4051561992e0 * t5253 + 0.10254018858216406658e4 * t5258 + t5263 + t5274 - t5278 + t5282 - t5288 - t5295 + t5298 + t5302 + 0.17315859105681463759e2 * t5303;
    (t8896, t8901)
}
