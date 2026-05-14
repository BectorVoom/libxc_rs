//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1328/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1328<F: Float>(t2464: F, t4826: F, t10798: F, t33031: F, t34031: F, t116676: F, t32990: F, t34097: F, t17182: F, t34260: F, t9664: F, t654: F, t7409: F, t34094: F, t5074: F, t32920: F, t7218: F) -> (F, F, F, F, F, F, F, F) {
    let t117020 = t4826 * t2464;
    let t117031 = 0.23148148148148148148e-2 * t33031 * t10798 * t34031;
    let t117033 = 0.23148148148148148148e-2 * t33031 * t116676;
    let t117062 = 0.69444444444444444446e-2 * t32990 * t34097;
    let t117065 = 0.69444444444444444446e-2 * t9664 * t17182 * t34260;
    let t117066 = t7409 * t654;
    let t117084 = t5074 * t34094;
    let t117086 = t32920 * t7218;
    (t117020, t117031, t117033, t117062, t117065, t117066, t117084, t117086)
}
