//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1009/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1009<F: Float>(t36042: F, t870: F, t2842: F, t7584: F, t10261: F, t10447: F, t10703: F, t1091: F, t112790: F, t112920: F, t114820: F, t143592: F, t143612: F, t144131: F, t1495: F, t15191: F, t152669: F, t15299: F, t15369: F, t15460: F, t1901: F, t24898: F, t25271: F, t2881: F, t29071: F, t29072: F, t29076: F, t29141: F, t29245: F, t29302: F, t34102: F, t34203: F, t34207: F, t36063: F, t36141: F, t36240: F, t4162: F, t4167: F, t4176: F, t4181: F, t44528: F, t56352: F, t6274: F, t684: F, t7032: F, t7101: F, t99034: F, t99186: F) -> (F,) {
    let t153821 = t870 * t36042;
    let t153830 = t2842 * t7584;
    let t153863 = -2.0 / 3.0 * t1901 * t15369 * t34207 * t4162 - 2.0 / 3.0 * t1901 * t15460 * t144131 * t4167 - 4.0 / 3.0 * t1901 * t112920 * t29141 - 4.0 * t1901 * t10261 * t1495 * t29072 - 4.0 / 3.0 * t1901 * t114820 * t29076 + t1901 * t10447 * t36141 / 9.0 + t1901 * t2881 * t143592 * t1091 / 9.0 - 2.0 / 9.0 * t1901 * t10703 * t34102 * t1091 + t1901 * t15191 * t34203 / 9.0 + t1901 * t2881 * t153821 * t684 / 9.0 + 2.0 * t1901 * t29071 * t143612 * t4176 + 4.0 / 3.0 * t1901 * t15369 * t153830 * t4181 - 4.0 / 3.0 * t1901 * t15369 * t24898 * t29302 + 2.0 / 9.0 * t1901 * t112790 * t6274 + 2.0 / 9.0 * t1901 * t99186 * t7032 + 2.0 / 9.0 * t1901 * t44528 * t36240 * t684 + 2.0 / 9.0 * t1901 * t99034 * t7101 - 4.0 / 3.0 * t1901 * t15460 * t25271 * t29245 - 4.0 / 9.0 * t1901 * t15299 * t152669 + 2.0 / 3.0 * t1901 * t56352 * t36063 * t684;
    (t153863,)
}
