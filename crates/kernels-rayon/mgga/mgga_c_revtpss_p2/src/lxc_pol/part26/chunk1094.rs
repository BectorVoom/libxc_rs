//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1094/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1094(t1940: f64, t2071: f64, t2257: f64, t2403: f64, t25198: f64, t25208: f64, t25211: f64, t25215: f64, t25446: f64, t25449: f64, t25452: f64, t26425: f64, t26581: f64, t26585: f64, t26590: f64, t30: f64, t4541: f64, t605: f64, t7010: f64, t7092: f64, t7428: f64, t7432: f64) -> f64 {
    let t26601 = 3.0_f64 * t4541 * t2071 * t25198 + 3.0_f64 * t2403 * t7428 * t7010 - 3.0_f64 * t26425 * t25208 + 3.0_f64 * t2403 * t2071 * t25211 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t25215 + t1940 * t26581 * t30 / 2.0_f64 - t1940 * t26585 * t7092 + t1940 * t7428 * t605 + t1940 * t26590 * t25446 - t1940 * t7432 * t25449 - t1940 * t7432 * t25452 / 2.0_f64 + t1940 * t2071 * t2257 / 2.0_f64;
    t26601
}
