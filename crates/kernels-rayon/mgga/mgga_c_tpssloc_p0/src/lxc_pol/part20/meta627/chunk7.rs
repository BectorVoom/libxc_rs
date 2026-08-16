//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2273/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2273(t2379: f64, t828: f64, t41115: f64, t4191: f64, t41107: f64, t4166: f64, t9670: f64, t831: f64, t13210: f64, t13228: f64, t13254: f64, t13333: f64, t13350: f64, t41130: f64, t41132: f64, t41134: f64, t41139: f64, t41237: f64, t41341: f64, t4167: f64, t4172: f64, t4178: f64, t9618: f64, t9642: f64, t9960: f64) -> f64 {
    let t47072 = t2379 * t828;
    let t47079 = t41115 * t4191;
    let t47080 = 119.0_f64 / 1152.0_f64 * t47079;
    let t47081 = t41107 * t4191;
    let t47092 = t4166 * t9670;
    let t47093 = t47092 * t831;
    let t47094 = 119.0_f64 / 4608.0_f64 * t47093;
    let t47097 = 5.0_f64 / 128.0_f64 * t4178 * t13350 * t13228 * t47072 + t9642 * t13210 / 256.0_f64 + t47080 - 7.0_f64 / 192.0_f64 * t47081 + 3.0_f64 / 512.0_f64 * t13254 * t13333 - 595.0_f64 / 3456.0_f64 * t41130 - 7.0_f64 / 4608.0_f64 * t41132 + 119.0_f64 / 4608.0_f64 * t41134 + t41139 + 7.0_f64 / 4608.0_f64 * t41237 - 119.0_f64 / 2304.0_f64 * t41341 + 5.0_f64 / 256.0_f64 * t4172 * t9618 - t47094 - t4167 * t9960 / 3072.0_f64;
    t47097
}
