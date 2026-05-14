//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 873/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk873<F: Float>(t33598: F, t51340: F, t13927: F, t33601: F, t1403: F, t140579: F, t140594: F, t1425: F, t193: F, t27882: F, t27997: F, t28002: F, t28006: F, t28042: F, t28461: F, t33277: F, t33494: F, t33499: F, t35287: F, t3683: F, t3837: F, t42500: F, t5996: F, t6002: F, t6749: F, t7558: F) -> (F, F, F) {
    let t149965 = t51340 * t33598;
    let t149967 = t13927 * t33601;
    let t149992 = -2.0 / 3.0 * t5996 * t35287 - 12.0 * t149965 + 8.0 * t149967 - t3683 * t7558 + t1403 * t193 * t1425 * t28461 / 3.0 + t6002 * t140594 * t28042 / 9.0 - 2.0 / 3.0 * t1403 * t193 * t27882 * t33277 - t140579 * t6749 / 18.0 - t33499 * t28002 / 18.0 - t33499 * t28006 / 18.0 + t33499 * t27997 - 4.0 * t6002 * t42500 * t33494 * t3837;
    (t149965, t149967, t149992)
}
