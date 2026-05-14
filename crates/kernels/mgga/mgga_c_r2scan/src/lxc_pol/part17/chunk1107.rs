//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1107/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1107<F: Float>(t37444: F, t39071: F, t39072: F, t44532: F, t44535: F, t44541: F, t44544: F, t44548: F, t44551: F, t44554: F, t44558: F, t44560: F, t44562: F, t44566: F, t44570: F, t37455: F, t37468: F, t39074: F, t39075: F, t39076: F, t40411: F, t42208: F, t42209: F, t42210: F, t43875: F, t43878: F, t44574: F, t44576: F, t44579: F, t44878: F) -> (F, F) {
    let t44988 = t44532 + 0.60975299583150056624e-3 * t37444 - t39071 + t44535 + t44541 + t44544 - t39072 - t44548 + t44551 - t44554 + t44558 - t44560 - t44562 + t44566 - t44570;
    let t44997 = -0.38422568777328955681e-2 * t37455 - t44574 - t44576 + t44579 - 0.2881692658299671676e-2 * t40411 + 0.1440846329149835838e-2 * t43875 - 0.20496175532535769482e-3 * t43878 + t39074 - t39075 - t39076 + t42208 - t42209 + t44878 + t42210 - 0.86737941314158990616e-4 * t37468;
    (t44988, t44997)
}
