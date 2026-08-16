//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1035/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1035(t22299: f64, t22301: f64, t22303: f64, t22305: f64, t22307: f64, t22309: f64, t22312: f64, t22315: f64, t22316: f64, t22319: f64, t22715: f64, t187: f64, t23375: f64) -> f64 {
    let t23376 = -t22299 + t22301 + t22303 - t22305 + t22307 - t22309 + t22312 - t22315 + t22316 - t22319 + t22715;
    let t23379 = t22299 - t22301 - t22303 + t22305 - t22307 + t22309 - t22312 + t22315 - t22316 + t22319 - t22715 + t187 * (t23375 + t23376);
    t23379
}
