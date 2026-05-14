//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1362/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1362<F: Float>(t1882: F, t26833: F, t27235: F, t8392: F, t38953: F, t6627: F, t105341: F, t105345: F, t105349: F, t105364: F, t105512: F, t105526: F, t105786: F, t11593: F, t12680: F, t12703: F, t13043: F, t13208: F, t13212: F, t17164: F, t1901: F, t23447: F, t23451: F, t23548: F, t27064: F, t27073: F, t40792: F, t49579: F, t50229: F, t51032: F) -> (F,) {
    let t106314 = 4.0 / 9.0 * t1882 * t26833;
    let t106319 = 4.0 / 81.0 * t8392 * t27235;
    let t106351 = t38953 * t6627;
    let t106356 = -t106314 + 10.0 / 81.0 * t1901 * t49579 * t105341 - t106319 + 2.0 / 9.0 * t1901 * t40792 * t23548 * t13043 - 4.0 / 9.0 * t1901 * t50229 * t27064 - 4.0 / 9.0 * t1901 * t12703 * t105364 - 2.0 / 9.0 * t1901 * t12703 * t105512 - 8.0 / 9.0 * t11593 * t12703 * t105526 - 2.0 / 9.0 * t1901 * t13208 * t105345 - 4.0 / 9.0 * t1901 * t13212 * t105349 - 8.0 / 9.0 * t11593 * t13208 * t105786 + 4.0 / 27.0 * t1901 * t51032 * t27073 + 2.0 / 27.0 * t1901 * t17164 * t23451 + 4.0 / 81.0 * t106351 + t1901 * t12680 * t23447 / 9.0;
    (t106356,)
}
