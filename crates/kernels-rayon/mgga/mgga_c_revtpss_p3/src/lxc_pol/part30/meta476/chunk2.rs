//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1800/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1800(t2408: f64, t30: f64, t605: f64, t890: f64, t2832: f64, t1940: f64, t1963: f64, t2257: f64, t2403: f64, t25198: f64, t25206: f64, t25208: f64, t25211: f64, t25215: f64, t25436: f64, t25440: f64, t25445: f64, t4541: f64, t7010: f64, t7087: f64, t7091: f64, t7092: f64) -> (f64, f64, f64, f64) {
    let t25446 = t30 * t2408;
    let t25449 = t605 * t890;
    let t25452 = t30 * t2832;
    let t25459 = 3.0_f64 * t4541 * t1963 * t25198 + 3.0_f64 * t2403 * t7087 * t7010 - 3.0_f64 * t25206 * t25208 + 3.0_f64 * t2403 * t1963 * t25211 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t25215 + t1940 * t25436 * t30 / 2.0_f64 - t1940 * t25440 * t7092 + t1940 * t7087 * t605 + t1940 * t25445 * t25446 - t1940 * t7091 * t25449 - t1940 * t7091 * t25452 / 2.0_f64 + t1940 * t1963 * t2257 / 2.0_f64;
    (t25446, t25449, t25452, t25459)
}
