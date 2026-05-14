//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 907/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk907<F: Float>(t10570: F, t10572: F, t10574: F, t10576: F, t10649: F, t15989: F, t15992: F, t15994: F, t15996: F, t16001: F, t16006: F, t16011: F, t16015: F, t16019: F, t16024: F, t16028: F, t16032: F) -> (F,) {
    let t16034 = -t10649 - 8.0 / 27.0 * t10570 + 2.0 / 27.0 * t10572 - 2.0 / 9.0 * t10574 + t10576 / 9.0 - 4.0 / 27.0 * t15989 + t15992 - t15994 - 22.0 / 9.0 * t15996 - 10.0 / 27.0 * t16001 + 4.0 / 3.0 * t16006 + 8.0 / 9.0 * t16011 - 2.0 / 9.0 * t16015 - 2.0 * t16019 - 8.0 / 3.0 * t16024 + 2.0 / 3.0 * t16028 + 2.0 / 3.0 * t16032;
    (t16034,)
}
