//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1213/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1213(t6290: f64, t7671: f64, t1876: f64, t2169: f64, t29220: f64, t29232: f64, t4533: f64, t911: f64, t92165: f64, t92168: f64, t92170: f64, t92339: f64, t92344: f64, t92351: f64, t93826: f64) -> f64 {
    let t99834 = t6290 * t7671;
    let t99835 = t911 * t29232 / 16.0_f64 - t92165 + t93826 + t92168 + t92170 + t92339 - t2169 * t4533 * t1876 / 8.0_f64 + t92344 - t92351 + t911 * t29220 / 16.0_f64 + t99834;
    t99835
}
