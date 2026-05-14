//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 723/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk723<F: Float>(t15752: F, t3506: F, t15756: F, t363: F, t4822: F, t12796: F, t2112: F, t358: F, t558: F, t12791: F, t11755: F, t11761: F, t12852: F, t12864: F, t12865: F, t17296: F, t17299: F, t17302: F, t17305: F, t17310: F, t17313: F, t17316: F, t17319: F, t17322: F, t17325: F, t3139: F, t462: F, t9178: F, t9202: F) -> (F,) {
    let t17328 = t3506 * t15752;
    let t17331 = t3506 * t15756;
    let t17334 = t4822 * t363;
    let t17335 = t12796 * t17334;
    let t17338 = t2112 * t358;
    let t17340 = t17338 * t4822 * t558;
    let t17343 = t12791 * t17334;
    let t17346 = -10.0 / 27.0 * t462 * t17296 - 8.0 / 9.0 * t3139 * t17299 + 2.0 / 3.0 * t462 * t17302 + t462 * t17305 / 3.0 - 8.0 / 27.0 * t12852 - t12864 + 4.0 / 9.0 * t12865 - t9178 - 2.0 / 9.0 * t17310 - 4.0 / 27.0 * t9202 - 2.0 / 3.0 * t462 * t17313 + t462 * t17316 / 3.0 + 2.0 / 3.0 * t462 * t17319 - 2.0 / 9.0 * t462 * t17322 - 2.0 / 3.0 * t462 * t17325 - 2.0 * t462 * t17328 + 8.0 / 3.0 * t3139 * t17331 + 4.0 / 9.0 * t11755 * t17335 - 4.0 / 3.0 * t11761 * t17340 - 4.0 / 3.0 * t11761 * t17343;
    (t17346,)
}
