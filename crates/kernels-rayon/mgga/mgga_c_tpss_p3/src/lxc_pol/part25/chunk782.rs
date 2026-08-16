//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 782/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk782(t189: f64, t5343: f64, t489: f64, t3182: f64, t3194: f64, t3196: f64, t3213: f64, t3216: f64, t3307: f64, t3310: f64, t5326: f64, t5327: f64, t5345: f64, t5346: f64) -> (f64, f64, f64) {
    let t5393 = t5343 * t189;
    let t5394 = t489 * t5393;
    let t5395 = t5394 + t5345 + t3307 + t3213 + t3216 + t5346 + t3310 - t5326 - t5327 + t3194 - t3196 - t3182;
    (t5393, t5394, t5395)
}
