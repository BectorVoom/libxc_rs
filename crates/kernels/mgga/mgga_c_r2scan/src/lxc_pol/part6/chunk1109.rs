//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1109/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1109<F: Float>(t1391: F, t20: F, t481: F, t506: F, t6068: F, t6159: F, t2146: F, t2182: F, t2185: F, t494: F, t113: F, t6086: F, t146: F, t6091: F, t774: F, t6094: F) -> (F, F, F, F, F, F, F) {
    let t19862 = t506 * t20 * t1391 * t481;
    let t19863 = t6159 * t6068 * t19862;
    let t19865 = t2182 * t2146;
    let t19866 = t2185 * t494;
    let t19867 = t19866 * t113;
    let t19869 = t19865 * t6086 * t19867;
    let t19872 = t146 * t6091 * t774;
    let t19873 = t19872 * t6094;
    (t19862, t19863, t19865, t19866, t19869, t19872, t19873)
}
