//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1004/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1004<F: Float>(t1181: F, t39491: F, t604: F, t7493: F, t1165: F, t5693: F, t7351: F, t8463: F, t30921: F, t35071: F, t35073: F, t35075: F, t35089: F, t35090: F, t35093: F, t35097: F, t35101: F, t37366: F, t37375: F, t39686: F, t39690: F, t39693: F, t39696: F, t39700: F) -> (F,) {
    let t39705 = t7493 * t1181 * t604 * t39491;
    let t39709 = t8463 * t1165 * t7351 * t5693;
    let t39711 = -t37366 + 0.42874018118069736972e-3 * t39686 + 0.33020496904084359671e-1 * t39690 + 0.183375e0 * t39693 + 0.13753125e0 * t39696 - t35071 - t35073 - t35075 - t30921 - t37375 - 0.38203125e-2 * t39700 + t35089 + 0.56606566121287473723e-2 * t35090 - t35093 - t35097 - t35101 - 0.21437009059034868486e-2 * t39705 + 0.28303283060643736861e-1 * t39709;
    (t39711,)
}
