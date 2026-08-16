//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 691/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk691(t2349: f64, t3308: f64, t187: f64, t3297: f64, t3180: f64, t3182: f64, t3192: f64, t3194: f64, t3196: f64, t3213: f64, t3216: f64, t3302: f64, t3304: f64, t3307: f64) -> (f64, f64, f64) {
    let t3310 = 0.10843581300301739842e-1_f64 * t3308 * t2349;
    let t3312 = 0.19751673498613801407e-1_f64 * t3297 * t187;
    let t3313 = t3302 + t3304 + t3307 + t3213 - t3216 + t3310 + t3312 - t3192 + t3194 - t3196 - t3180 - t3182;
    (t3310, t3312, t3313)
}
