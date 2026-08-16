//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2805/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2805(t5619: f64, t9671: f64, t13222: f64, t13229: f64, t13352: f64, t16976: f64, t20981: f64, t2701: f64, t2703: f64, t4178: f64, t4281: f64, t4291: f64, t47269: f64, t47271: f64, t47273: f64, t47276: f64, t47279: f64, t47283: f64, t5585: f64, t58090: f64, t59251: f64, t59255: f64, t59257: f64, t59259: f64, t59261: f64, t59263: f64, t59265: f64, t59267: f64, t820: f64, t843: f64) -> f64 {
    let t59276 = t9671 * t5619;
    let t59278 = -119.0_f64 / 864.0_f64 * t47269 + 7.0_f64 / 576.0_f64 * t47271 + 7.0_f64 / 288.0_f64 * t47273 - 119.0_f64 / 864.0_f64 * t47276 + 7.0_f64 / 288.0_f64 * t47279 + 7.0_f64 / 576.0_f64 * t47283 - t4178 * t13222 * t5585 * t13229 / 64.0_f64 + 5.0_f64 / 384.0_f64 * t843 * t2701 * t820 * t58090 - 35.0_f64 / 576.0_f64 * t59251 + 5.0_f64 / 768.0_f64 * t16976 * t2703 + 7.0_f64 / 576.0_f64 * t59255 + 7.0_f64 / 576.0_f64 * t59257 - 119.0_f64 / 1728.0_f64 * t59259 + 7.0_f64 / 288.0_f64 * t59261 - 119.0_f64 / 3456.0_f64 * t59263 - 5.0_f64 / 192.0_f64 * t4291 * t59265 * t59267 * t13352 + 5.0_f64 / 96.0_f64 * t4281 * t59265 * t59267 * t20981 * t13229 - 119.0_f64 / 13824.0_f64 * t59276;
    t59278
}
