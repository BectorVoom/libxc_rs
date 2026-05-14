//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1410/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1410<F: Float>(t31703: F, t8392: F, t1882: F, t31695: F, t10683: F, t10703: F, t1091: F, t112725: F, t113903: F, t114869: F, t114886: F, t125670: F, t126795: F, t1508: F, t15299: F, t1901: F, t19399: F, t19490: F, t19593: F, t28843: F, t29185: F, t296: F, t31936: F, t4181: F, t4255: F, t446: F, t56098: F, t56352: F, t684: F, t7036: F, t72190: F, t835: F, t99238: F, t99867: F) -> (F,) {
    let t128504 = t8392 * t31703;
    let t128513 = t1882 * t31695;
    let t128538 = 8.0 / 3.0 * t1901 * t72190 * t7036 * t4181 - 4.0 / 9.0 * t1901 * t15299 * t126795 + 4.0 / 27.0 * t128504 - 2.0 / 9.0 * t1901 * t56098 * t29185 - 2.0 / 9.0 * t1901 * t99238 * t19593 + 4.0 / 27.0 * t99867 - 2.0 / 9.0 * t128513 - t114869 + 2.0 / 3.0 * t1901 * t56352 * t31936 * t684 - 2.0 / 9.0 * t1901 * t10703 * t112725 * t4255 - 8.0 / 27.0 * t114886 - 2.0 / 9.0 * t446 * t835 * t28843 * t1091 - 2.0 * t446 * t10683 * t1508 * t19399 - 2.0 / 3.0 * t446 * t296 * t125670 - 10.0 / 81.0 * t1901 * t113903 * t19490;
    (t128538,)
}
