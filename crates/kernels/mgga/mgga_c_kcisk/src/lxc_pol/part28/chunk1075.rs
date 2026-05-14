//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1075/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1075<F: Float>(t10570: F, t12002: F, t15989: F, t15991: F, t15996: F, t18616: F, t22564: F, t22567: F, t22570: F, t22573: F, t22575: F, t22578: F, t22581: F, t22583: F, t22586: F, t22589: F, t22594: F) -> (F,) {
    let t24661 = -t12002 - 0.76103703703703703703e-2 * t10570 - 0.1522074074074074074e-1 * t15989 + 0.761037037037037037e-2 * t15991 - t18616 - 0.2283111111111111111e-1 * t15996 + 0.3805185185185185185e-2 * t22564 - 0.19025925925925925925e-1 * t22567 + 0.68493333333333333331e-1 * t22570 + 0.4566222222222222222e-1 * t22573 - 0.11415555555555555555e-1 * t22575 - 0.10274e0 * t22578 - 0.13698666666666666666e0 * t22581 + 0.57077777777777777777e-2 * t22583 - 0.11415555555555555555e-1 * t22586 + 0.34246666666666666666e-1 * t22589 - 0.17123333333333333333e-1 * t22594;
    (t24661,)
}
