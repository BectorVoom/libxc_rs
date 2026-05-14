//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 723/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk723<F: Float>(t8643: F, t960: F, t8649: F, t8652: F, t965: F, t1850: F, t7715: F, t696: F, t7718: F, t5136: F, t4811: F, t8948: F, t1333: F, t8667: F, t1772: F, t8793: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23251 = t960 * t8643;
    let t23253 = t960 * t8649;
    let t23255 = t965 * t8652;
    let t23259 = t1850 * t7715;
    let t23261 = t696 * t7718;
    let t23263 = t5136 * t7718;
    let t23286 = t4811 * t8948;
    let t23320 = t1333 * t8667;
    let t23326 = t8793 * t1772;
    (t23251, t23253, t23255, t23259, t23261, t23263, t23286, t23320, t23326)
}
