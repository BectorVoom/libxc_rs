//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1040/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1040<F: Float>(t14026: F, t14481: F, t14483: F, t14485: F, t14487: F, t14489: F, t14491: F, t14493: F, t14495: F, t14499: F, t14502: F, t14506: F, t14520: F, t14030: F, t14508: F, t14510: F, t14512: F, t14514: F, t14516: F, t14518: F, t14523: F, t14525: F) -> (F, F) {
    let t15049 = -t14481 / 192.0 + t14483 / 48.0 - t14485 / 384.0 + t14487 / 96.0 - t14489 / 384.0 + t14491 / 48.0 - t14493 / 192.0 - t14495 / 48.0 + t14499 / 128.0 - t14026 + t14502 / 384.0;
    let t15050 = 7.0 / 576.0 * t14506;
    let t15057 = 7.0 / 144.0 * t14520;
    let t15060 = -t14030 + t15050 - t14508 / 48.0 + t14510 / 24.0 + t14512 / 24.0 + t14514 / 24.0 + 5.0 / 192.0 * t14516 + t14518 / 96.0 - t15057 - t14523 / 48.0 + t14525 / 192.0;
    (t15049, t15060)
}
