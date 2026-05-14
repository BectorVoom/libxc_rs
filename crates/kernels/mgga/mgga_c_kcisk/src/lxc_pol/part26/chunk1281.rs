//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1281/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1281<F: Float>(t20233: F, t32087: F, t33409: F, t113576: F, t9426: F, t1308: F, t388: F, t52538: F, t1333: F, t33577: F, t33541: F, t3748: F, t33363: F, t19734: F, t33367: F, t3805: F, t9818: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t114395 = t32087 * t20233 * t33409;
    let t114405 = 0.26805555555555555556e-2 * t9426 * t113576;
    let t114407 = t52538 * t388 * t1308;
    let t114437 = t1333 * t33577;
    let t114438 = 0.33163888888888888888e-2 * t114437;
    let t114439 = t3748 * t33541;
    let t114440 = 0.14739506172839506172e-2 * t114439;
    let t114453 = t3748 * t33363;
    let t114454 = 0.22109259259259259258e-2 * t114453;
    let t114455 = t19734 * t33367;
    let t114464 = t3805 * t9818;
    (t114395, t114405, t114407, t114437, t114438, t114439, t114440, t114453, t114454, t114455, t114464)
}
