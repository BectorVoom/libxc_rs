//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1134/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1134(t10610: f64, t3263: f64, t42392: f64, t11479: f64, t3275: f64, t7040: f64, t14160: f64, t3245: f64, t3270: f64, t3269: f64, t2850: f64, t6967: f64) -> (f64, f64, f64, f64) {
    let t42395 = 3.0_f64 * t10610 * t3263 * t42392;
    let t42398 = t3275 * t11479 * t7040 / 2.0_f64;
    let t42399 = t14160 * t3245;
    let t42400 = t3270 * t42399;
    let t42402 = t3269 * t42400 / 2.0_f64;
    let t42403 = t6967 * t2850;
    (t42395, t42398, t42402, t42403)
}
