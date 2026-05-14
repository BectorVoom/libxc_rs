//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1351/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1351<F: Float>(t113430: F, t9836: F, t32266: F, t34869: F, t26416: F, t4204: F, t9497: F, t1415: F, t27130: F, t27175: F, t33652: F, t109279: F, t8271: F, t27140: F, t32278: F, t27089: F) -> (F, F, F, F, F, F, F, F) {
    let t119733 = t113430 * t9836;
    let t119735 = t32266 * t34869;
    let t119738 = t9497 * t4204 * t26416;
    let t119740 = t1415 * t27130;
    let t119742 = t33652 * t27175;
    let t119744 = t109279 * t8271;
    let t119746 = t32278 * t27140;
    let t119748 = t32278 * t27089;
    (t119733, t119735, t119738, t119740, t119742, t119744, t119746, t119748)
}
