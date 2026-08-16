//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2023/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2023(t10569: f64, t10574: f64, t10566: f64, t10568: f64, t11075: f64, t14340: f64, t14343: f64, t14345: f64, t14352: f64, t14353: f64, t14364: f64, t14365: f64, t14372: f64, t14373: f64, t14374: f64, t14375: f64, t1544: f64, t1940: f64, t198: f64, t2403: f64, t2404: f64, t2430: f64, t2832: f64, t4343: f64, t4546: f64, t4556: f64, t775: f64, t9394: f64) -> (f64, f64, f64) {
    let t14379 = 0.4883052614935078681e-3_f64 * t10569;
    let t14380 = 0.18311447306006545054e-3_f64 * t10574;
    let t14381 = 3.0_f64 * t11075 * t1544 * t2403 + 6.0_f64 * t14353 * t2403 * t775 - 6.0_f64 * t14365 * t2403 * t4556 + 6.0_f64 * t14375 * t1544 * t198 - t1940 * t2832 * t4556 + 6.0_f64 * t2403 * t2404 * t4343 + 3.0_f64 * t2403 * t2430 * t4546 + t10566 - t10568 + t14340 + t14343 + t14345 + t14352 + t14364 + t14372 + t14373 + t14374 + t14379 - t14380 + t9394;
    (t14379, t14380, t14381)
}
