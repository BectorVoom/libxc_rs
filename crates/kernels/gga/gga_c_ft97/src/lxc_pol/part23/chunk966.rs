//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 966/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk966<F: Float>(t1253: F, t6261: F, t193: F, t4299: F, t6353: F, t1476: F, t2665: F, t684: F, t1466: F, t25430: F, t25459: F, t28983: F, t28987: F, t28990: F, t28993: F, t28997: F, t29000: F, t29002: F, t29006: F, t29008: F, t6210: F, t6216: F, t6219: F, t6225: F, t6963: F, t6967: F, t7028: F) -> (F, F, F, F, F, F) {
    let t29016 = t6261 * t1253;
    let t29017 = t193 * t29016;
    let t29020 = t6353 * t4299;
    let t29024 = t1476 * t1253;
    let t29026 = t2665 * t29024 * t684;
    let t29029 = -2.0 * t28983 - t6216 * t28987 / 18.0 + t28990 / 54.0 - t6216 * t28993 / 18.0 - t6216 * t28997 / 18.0 + t29000 * t29002 / 9.0 + t6216 * t29006 - t29008 * t6219 / 18.0 - t25459 * t6967 / 18.0 - t25430 / 18.0 + t6210 * t7028 / 6.0 + t1466 * t29017 / 6.0 - 2.0 * t29020 - t6963 * t6225 / 3.0 - t6216 * t29026 / 18.0;
    (t29016, t29017, t29020, t29024, t29026, t29029)
}
