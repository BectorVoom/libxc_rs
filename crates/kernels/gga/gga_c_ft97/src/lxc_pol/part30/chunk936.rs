//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 936/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk936<F: Float>(t140653: F, t3864: F, t10052: F, t3972: F, t7546: F, t10002: F, t35737: F, t24412: F, t27986: F, t1403: F, t141560: F, t193: F, t24231: F, t27992: F, t28043: F, t33259: F, t33499: F, t35285: F, t35297: F, t35751: F, t35779: F, t5996: F, t6002: F, t6064: F, t6745: F, t684: F, t771: F) -> (F, F, F, F, F) {
    let t151387 = t140653 * t3864;
    let t151405 = t10052 * t7546 * t3972;
    let t151407 = t10002 * t35737;
    let t151409 = t24412 * t27986;
    let t151411 = -t141560 / 18.0 + t6745 * t33259 / 3.0 - t5996 * t35297 / 3.0 + 4.0 * t151387 + 2.0 / 9.0 * t6002 * t24231 * t27992 + t33499 * t28043 / 9.0 + 2.0 / 9.0 * t6002 * t24231 * t35285 * t684 + t35779 * t6064 / 6.0 + t1403 * t193 * t35751 * t771 / 6.0 - 12.0 * t151405 + 8.0 * t151407 + 8.0 * t151409;
    (t151387, t151405, t151407, t151409, t151411)
}
