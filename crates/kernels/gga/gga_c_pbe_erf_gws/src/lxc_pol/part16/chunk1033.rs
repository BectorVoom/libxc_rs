//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1033/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1033<F: Float>(t14506: F, t14520: F, t14030: F, t14508: F, t14510: F, t14512: F, t14514: F, t14516: F, t14518: F, t14523: F, t14525: F, t14551: F, t14043: F, t14048: F, t14529: F, t14531: F, t14533: F, t14536: F, t14539: F, t14542: F, t14544: F, t14549: F) -> (F, F) {
    let t15050 = 7.0 / 576.0 * t14506;
    let t15057 = 7.0 / 144.0 * t14520;
    let t15060 = -t14030 + t15050 - t14508 / 48.0 + t14510 / 24.0 + t14512 / 24.0 + t14514 / 24.0 + 5.0 / 192.0 * t14516 + t14518 / 96.0 - t15057 - t14523 / 48.0 + t14525 / 192.0;
    let t15070 = 7.0 / 576.0 * t14551;
    let t15071 = -t14529 / 384.0 - t14531 / 96.0 - t14533 / 24.0 - t14536 / 24.0 - t14539 / 48.0 + t14043 - t14542 / 24.0 + t14544 / 384.0 + t14048 + t14549 / 8.0 - t15070;
    (t15060, t15071)
}
