//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 740/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk740<F: Float>(t19030: F, t2665: F, t446: F, t10279: F, t10400: F, t14636: F, t14638: F, t14640: F, t14658: F, t14684: F, t14718: F, t14903: F, t15111: F, t15116: F, t18999: F, t19004: F, t19008: F, t19013: F, t19018: F, t19022: F, t19025: F, t19028: F) -> (F, F) {
    let t19031 = t2665 * t19030;
    let t19032 = t446 * t19031;
    let t19034 = -t14636 - t14638 + t14640 - t14658 - t14684 - 2.0 / 27.0 * t10400 - 2.0 / 81.0 * t10279 - t15111 - 2.0 / 27.0 * t14718 - 2.0 / 9.0 * t18999 - 2.0 / 9.0 * t19004 + 2.0 / 27.0 * t19008 - t15116 + t14903 + t19013 / 18.0 - t19018 / 9.0 - t19022 / 9.0 - t19025 / 3.0 - 4.0 / 9.0 * t19028 + t19032 / 18.0;
    (t19032, t19034)
}
