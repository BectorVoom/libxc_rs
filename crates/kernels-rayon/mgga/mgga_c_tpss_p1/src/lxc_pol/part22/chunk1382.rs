//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1382/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1382(t1795: f64, t2105: f64, t10456: f64, t1165: f64, t13146: f64, t1338: f64, t18898: f64, t20289: f64, t20294: f64, t20319: f64, t2056: f64, t3537: f64, t4347: f64, t62230: f64, t6323: f64, t645: f64, t67250: f64, t67316: f64, t67519: f64, t67538: f64, t67541: f64) -> (f64, f64) {
    let t67552 = t1795 * t2105;
    let t67557 = 4.0_f64 * t10456 * t6323 + 2.0_f64 * t1165 * t67538 + 2.0_f64 * t13146 * t6323 + 2.0_f64 * t1338 * t62230 + 4.0_f64 * t1338 * t67250 + 2.0_f64 * t1338 * t67552 + 4.0_f64 * t18898 * t3537 + 2.0_f64 * t20289 * t2105 + 4.0_f64 * t20294 * t3537 + 4.0_f64 * t20319 * t2056 + 4.0_f64 * t20319 * t4347 + 4.0_f64 * t645 * t67541 + 2.0_f64 * t67316 + t67519;
    (t67552, t67557)
}
