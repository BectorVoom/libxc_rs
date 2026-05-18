//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 972/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk972<F: Float>(t19020: F, t2665: F, t446: F, t17744: F, t835: F, t17780: F, t3281: F, t4973: F, t824: F, t10279: F, t10400: F, t14636: F, t14638: F, t14640: F, t14658: F, t14684: F, t14718: F, t14903: F, t15111: F, t15116: F, t18999: F, t19004: F, t19008: F, t19013: F, t19018: F) -> (F, F, F, F, F, F) {
    let t19021 = t2665 * t19020;
    let t19022 = t446 * t19021;
    let t19024 = t835 * t17744;
    let t19025 = t446 * t19024;
    let t19027 = t835 * t17780;
    let t19028 = t3281 * t19027;
    let t19030 = t4973 * t824;
    let t19031 = t2665 * t19030;
    let t19032 = t446 * t19031;
    let t19034 = -t14636 - t14638 + t14640 - t14658 - t14684 - F::new(2.0) / F::new(27.0) * t10400 - F::new(2.0) / F::new(81.0) * t10279 - t15111 - F::new(2.0) / F::new(27.0) * t14718 - F::new(2.0) / F::new(9.0) * t18999 - F::new(2.0) / F::new(9.0) * t19004 + F::new(2.0) / F::new(27.0) * t19008 - t15116 + t14903 + t19013 / F::new(18.0) - t19018 / F::new(9.0) - t19022 / F::new(9.0) - t19025 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t19028 + t19032 / F::new(18.0);
    (t19022, t19025, t19028, t19030, t19032, t19034)
}
