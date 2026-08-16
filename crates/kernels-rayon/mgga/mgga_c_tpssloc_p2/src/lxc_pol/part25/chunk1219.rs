//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1219/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1219(t85166: f64, t870: f64, t1877: f64, t2057: f64, t2058: f64, t22961: f64, t22968: f64, t23296: f64, t23299: f64, t23302: f64, t24191: f64, t24335: f64, t24339: f64, t25: f64, t2522: f64, t26563: f64, t606: f64, t7110: f64, t7114: f64, t81470: f64, t81476: f64, t81486: f64, t81509: f64, t81513: f64, t81548: f64, t82320: f64, t82330: f64, t84797: f64, t84800: f64) -> (f64, f64) {
    let t85167 = t85166 * t870;
    let t85187 = -9.0_f64 * t84797 * t22961 + 3.0_f64 * t1877 * t84800 * t23296 - 9.0_f64 * t24191 * t81548 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t81509 + 3.0_f64 * t82320 * t2058 - 9.0_f64 * t26563 * t81486 - 3.0_f64 / 2.0_f64 * t1877 * t7114 * t81513 + 9.0_f64 * t26563 * t81470 + t1877 * t85167 * t25 / 2.0_f64 + 9.0_f64 * t24191 * t81476 + 3.0_f64 / 2.0_f64 * t1877 * t24335 * t606 + 9.0_f64 / 2.0_f64 * t2522 * t7110 * t22968 - 3.0_f64 / 2.0_f64 * t1877 * t24339 * t23302 - 9.0_f64 / 2.0_f64 * t24191 * t82330 - 3.0_f64 * t1877 * t24339 * t23299;
    (t85167, t85187)
}
