//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1396/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1396<F: Float>(t31714: F, t8392: F, t10261: F, t114683: F, t114694: F, t114707: F, t1240: F, t127034: F, t1508: F, t1901: F, t19240: F, t19362: F, t19423: F, t19815: F, t25188: F, t2862: F, t29071: F, t29072: F, t29076: F, t29189: F, t29245: F, t319: F, t31956: F, t4246: F, t446: F, t5424: F, t6273: F, t6278: F, t684: F, t69996: F, t72163: F, t835: F, t840: F, t99238: F) -> (F,) {
    let t128143 = t8392 * t31714;
    let t128145 = 2.0 / 3.0 * t446 * t840 * t4246 * t29245 - t114683 - 4.0 * t1901 * t10261 * t1240 * t29072 - 4.0 / 3.0 * t1901 * t69996 * t29076 - 2.0 * t1901 * t29071 * t6273 * t19362 - 4.0 / 9.0 * t1901 * t72163 * t29189 - 2.0 / 9.0 * t1901 * t99238 * t19815 - t114694 - 2.0 / 3.0 * t446 * t840 * t25188 * t19423 + 4.0 / 3.0 * t446 * t2862 * t319 * t127034 + 2.0 / 3.0 * t446 * t2862 * t5424 * t6278 - t114707 - t446 * t840 * t1508 * t19240 / 3.0 - t446 * t835 * t31956 * t684 / 9.0 - 2.0 / 27.0 * t128143;
    (t128145,)
}
