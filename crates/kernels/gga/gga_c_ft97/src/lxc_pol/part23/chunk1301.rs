//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1301/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1301<F: Float>(t30896: F, t5999: F, t263: F, t30859: F, t107782: F, t109809: F, t1403: F, t18206: F, t18497: F, t193: F, t2354: F, t24204: F, t24245: F, t27971: F, t28010: F, t28026: F, t28036: F, t28037: F, t28042: F, t30871: F, t30919: F, t42500: F, t4965: F, t51340: F, t5179: F, t6002: F, t6003: F, t6008: F, t6068: F, t684: F, t713: F, t9744: F, t98168: F) -> (F,) {
    let t125337 = t30896 * t5999;
    let t125347 = t30859 * t263;
    let t125362 = 4.0 / 27.0 * t28010 * t28036 * t28037 * t18497 - t6002 * t98168 * t30919 * t684 / 3.0 + 2.0 / 9.0 * t6002 * t107782 * t28026 + t30896 * t6068 / 6.0 + 2.0 / 9.0 * t6002 * t107782 * t28042 - t125337 / 18.0 - t6002 * t9744 * t24245 * t4965 / 27.0 - 4.0 * t6002 * t42500 * t6003 * t18206 - t6002 * t2354 * t125347 * t684 / 18.0 - t24204 * t30871 / 18.0 + 4.0 / 27.0 * t109809 - 24.0 * t51340 * t27971 - t1403 * t193 * t6008 * t5179 * t713 / 3.0;
    (t125362,)
}
