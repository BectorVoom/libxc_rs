//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 932/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk932<F: Float>(t16517: F, t1676: F, t1685: F, t15993: F, t10569: F, t10570: F, t10572: F, t10574: F, t10576: F, t15989: F, t15991: F, t15996: F, t16001: F, t16006: F, t16011: F, t16015: F, t16019: F, t16024: F, t16028: F, t16032: F) -> (F, F) {
    let t16519 = t1676 * t16517 * t1685;
    let t16528 = 0.23744444444444444444e-1 * t15993;
    let t16538 = -t10569 - 0.15829629629629629629e-1 * t10570 + 0.39574074074074074073e-2 * t10572 - 0.11872222222222222222e-1 * t10574 + 0.5936111111111111111e-2 * t10576 - 0.79148148148148148146e-2 * t15989 + 0.79148148148148148146e-2 * t15991 - t16528 - 0.13059444444444444444e0 * t15996 - 0.19787037037037037037e-1 * t16001 + 0.71233333333333333332e-1 * t16006 + 0.47488888888888888888e-1 * t16011 - 0.11872222222222222222e-1 * t16015 - 0.10685e0 * t16019 - 0.14246666666666666666e0 * t16024 + 0.35616666666666666666e-1 * t16028 + 0.35616666666666666666e-1 * t16032;
    (t16519, t16538)
}
