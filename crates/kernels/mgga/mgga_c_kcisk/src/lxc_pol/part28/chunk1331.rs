//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1331/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1331<F: Float>(t1333: F, t34170: F, t34177: F, t34054: F, t34161: F, t4811: F, t34222: F, t34233: F, t9660: F, t33001: F, t7218: F, t2454: F, t5283: F, t17982: F, t654: F, t7400: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t117193 = t1333 * t34170;
    let t117194 = 0.22109259259259259258e-2 * t117193;
    let t117195 = t1333 * t34177;
    let t117203 = t1333 * t34054;
    let t117204 = 0.33163888888888888888e-2 * t117203;
    let t117205 = t4811 * t34161;
    let t117206 = 0.66327777777777777776e-2 * t117205;
    let t117207 = t4811 * t34222;
    let t117211 = 0.69444444444444444446e-2 * t34233 * t9660;
    let t117248 = t33001 * t7218;
    let t117294 = t5283 * t2454;
    let t117310 = t17982 * t654;
    let t117327 = t7400 * t654;
    (t117193, t117194, t117195, t117203, t117204, t117205, t117206, t117207, t117211, t117248, t117294, t117310, t117327)
}
