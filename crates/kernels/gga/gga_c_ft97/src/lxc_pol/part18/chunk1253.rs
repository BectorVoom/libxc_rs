//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1253/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1253<F: Float>(t1339: F, t8326: F, t1882: F, t26420: F, t26425: F, t26151: F, t488: F, t7750: F, t102431: F, t10974: F, t11568: F, t1876: F, t1901: F, t26134: F, t3183: F, t39285: F, t446: F, t452: F, t47089: F, t5710: F, t6469: F, t6534: F, t83: F, t91945: F, t91951: F, t91980: F, t91982: F, t91993: F, t93636: F) -> (F,) {
    let t103472 = t8326 * t1339;
    let t103486 = 2.0 / 9.0 * t1882 * t26420;
    let t103488 = 2.0 / 9.0 * t1882 * t26425;
    let t103490 = 4.0 / 9.0 * t1882 * t26151;
    let t103491 = t7750 * t488;
    let t103508 = 4.0 / 27.0 * t1901 * t103472 * t10974 + 4.0 / 3.0 * t446 * t83 * t102431 + t446 * t452 * t5710 * t11568 / 3.0 - 2.0 / 9.0 * t91945 - 2.0 / 27.0 * t91951 - t103486 - t103488 - t103490 + 4.0 * t1901 * t103491 * t6469 * t1876 + t1901 * t39285 * t6534 / 9.0 + 2.0 / 9.0 * t1901 * t93636 * t3183 + 2.0 / 27.0 * t91980 + 4.0 / 27.0 * t91982 - 4.0 / 9.0 * t1901 * t47089 * t26134 + t91993 / 9.0;
    (t103508,)
}
