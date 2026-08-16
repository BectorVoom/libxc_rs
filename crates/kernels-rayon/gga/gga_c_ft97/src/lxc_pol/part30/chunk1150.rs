//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1150/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1150(t36042: f64, t870: f64, t2842: f64, t7584: f64, t10261: f64, t10447: f64, t10703: f64, t1091: f64, t112790: f64, t112920: f64, t114820: f64, t143592: f64, t143612: f64, t144131: f64, t1495: f64, t15191: f64, t152669: f64, t15299: f64, t15369: f64, t15460: f64, t1901: f64, t24898: f64, t25271: f64, t2881: f64, t29071: f64, t29072: f64, t29076: f64, t29141: f64, t29245: f64, t29302: f64, t34102: f64, t34203: f64, t34207: f64, t36063: f64, t36141: f64, t36240: f64, t4162: f64, t4167: f64, t4176: f64, t4181: f64, t44528: f64, t56352: f64, t6274: f64, t684: f64, t7032: f64, t7101: f64, t99034: f64, t99186: f64) -> f64 {
    let t153821 = t870 * t36042;
    let t153830 = t2842 * t7584;
    let t153863 = -2.0_f64 / 3.0_f64 * t1901 * t15369 * t34207 * t4162 - 2.0_f64 / 3.0_f64 * t1901 * t15460 * t144131 * t4167 - 4.0_f64 / 3.0_f64 * t1901 * t112920 * t29141 - 4.0_f64 * t1901 * t10261 * t1495 * t29072 - 4.0_f64 / 3.0_f64 * t1901 * t114820 * t29076 + t1901 * t10447 * t36141 / 9.0_f64 + t1901 * t2881 * t143592 * t1091 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t10703 * t34102 * t1091 + t1901 * t15191 * t34203 / 9.0_f64 + t1901 * t2881 * t153821 * t684 / 9.0_f64 + 2.0_f64 * t1901 * t29071 * t143612 * t4176 + 4.0_f64 / 3.0_f64 * t1901 * t15369 * t153830 * t4181 - 4.0_f64 / 3.0_f64 * t1901 * t15369 * t24898 * t29302 + 2.0_f64 / 9.0_f64 * t1901 * t112790 * t6274 + 2.0_f64 / 9.0_f64 * t1901 * t99186 * t7032 + 2.0_f64 / 9.0_f64 * t1901 * t44528 * t36240 * t684 + 2.0_f64 / 9.0_f64 * t1901 * t99034 * t7101 - 4.0_f64 / 3.0_f64 * t1901 * t15460 * t25271 * t29245 - 4.0_f64 / 9.0_f64 * t1901 * t15299 * t152669 + 2.0_f64 / 3.0_f64 * t1901 * t56352 * t36063 * t684;
    t153863
}
