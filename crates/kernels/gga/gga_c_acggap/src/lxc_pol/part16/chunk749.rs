//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 749/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk749<F: Float>(t8607: F, t8619: F, t8625: F, t8650: F, t8680: F, t8682: F, t8684: F, t8690: F, t8694: F, t8706: F, t8710: F, t8712: F, t8714: F, t8716: F, t8718: F, t8722: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9222 = 0.42874018118069736972e-3 * t8607;
    let t9226 = 0.28015625e-1 * t8619;
    let t9228 = 7.0 / 144.0 * t8625;
    let t9239 = 0.10718504529517434243e-2 * t8650;
    let t9248 = 11.0 / 192.0 * t8680;
    let t9249 = 11.0 / 576.0 * t8682;
    let t9250 = 7.0 / 72.0 * t8684;
    let t9252 = 0.21437009059034868486e-3 * t8690;
    let t9254 = 0.17149607247227894789e-2 * t8694;
    let t9261 = 0.17149607247227894789e-2 * t8706;
    let t9263 = 0.34299214494455789578e-2 * t8710;
    let t9264 = 0.80031500487063509015e-2 * t8712;
    let t9265 = 0.80031500487063509015e-2 * t8714;
    let t9266 = 0.16006300097412701803e-1 * t8716;
    let t9267 = 0.34299214494455789578e-2 * t8718;
    let t9269 = 0.12862205435420921092e-2 * t8722;
    (t9222, t9226, t9228, t9239, t9248, t9249, t9250, t9252, t9254, t9261, t9263, t9264, t9265, t9266, t9267, t9269)
}
