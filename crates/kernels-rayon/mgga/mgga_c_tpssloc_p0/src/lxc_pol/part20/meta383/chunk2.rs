//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1748/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1748(t13263: f64, t4180: f64, t4181: f64, t13225: f64, t13231: f64, t13234: f64, t13237: f64, t13244: f64, t13248: f64, t13251: f64, t13254: f64, t13260: f64, t13262: f64, t2643: f64, t2649: f64, t4178: f64, t4184: f64, t4191: f64, t4240: f64, t9639: f64, t9642: f64, t9668: f64, t9672: f64, t9675: f64, t9679: f64, t9986: f64, t9988: f64, t9994: f64) -> (f64, f64) {
    let t13265 = t4180 * t4181 * t13263;
    let t13268 = -7.0_f64 / 576.0_f64 * t9639 - 7.0_f64 / 2304.0_f64 * t9668 - 119.0_f64 / 6912.0_f64 * t9672 + 7.0_f64 / 2304.0_f64 * t9675 + 7.0_f64 / 4608.0_f64 * t9679 + 7.0_f64 / 4608.0_f64 * t9986 - 35.0_f64 / 1152.0_f64 * t9988 + 7.0_f64 / 576.0_f64 * t9994 + t2643 * t13225 / 384.0_f64 - t4178 * t13231 / 192.0_f64 + 119.0_f64 / 13824.0_f64 * t13234 - t13237 + t9642 * t4191 / 384.0_f64 - t9642 * t4240 / 1536.0_f64 + t4178 * t13244 / 768.0_f64 + t4178 * t13248 / 1536.0_f64 + t13251 * t2649 / 384.0_f64 + t13254 * t4184 / 768.0_f64 - t13260 - t13262 * t13265 / 512.0_f64;
    (t13265, t13268)
}
