//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1214/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1214<F: Float>(t7038: F, t8232: F, t1495: F, t799: F, t10696: F, t14615: F, t15191: F, t15284: F, t15369: F, t15433: F, t15460: F, t15462: F, t1901: F, t24898: F, t25271: F, t25373: F, t29055: F, t29123: F, t29127: F, t29128: F, t29129: F, t29130: F, t4181: F, t56180: F, t56522: F, t56815: F, t6274: F, t6361: F, t6386: F, t863: F, t98788: F, t98790: F, t98800: F) -> (F,) {
    let t112904 = t8232 * t7038;
    let t112920 = t799 * t1495;
    let t112950 = 8.0 / 27.0 * t112904 - 4.0 / 3.0 * t1901 * t15369 * t24898 * t15284 - 4.0 / 3.0 * t1901 * t15460 * t29055 * t14615 + 2.0 / 9.0 * t1901 * t56522 * t6274 - 2.0 / 9.0 * t1901 * t15191 * t25373 - 4.0 / 3.0 * t1901 * t112920 * t15462 + 2.0 / 9.0 * t1901 * t56180 * t6361 - 4.0 * t1901 * t29128 * t29129 * t14615 - 2.0 / 3.0 * t1901 * t15460 * t25271 * t15433 - 4.0 * t1901 * t29127 * t863 * t29130 - 4.0 * t1901 * t29128 * t10696 * t6386 * t4181 - 4.0 / 3.0 * t1901 * t56815 * t29123 + 8.0 / 27.0 * t98788 + 16.0 / 27.0 * t98790 + t98800 / 9.0;
    (t112950,)
}
