//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1316/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1316(t10416: f64, t1312: f64, t13425: f64, t13426: f64, t13429: f64, t13435: f64, t13440: f64, t13514: f64, t1518: f64, t2322: f64, t2371: f64, t4248: f64, t4292: f64, t5523: f64, t670: f64) -> f64 {
    let t13517 = 2.0_f64 * t10416 * t1518 + 2.0_f64 * t1312 * t13514 + 4.0_f64 * t13426 * t670 + 4.0_f64 * t13435 * t1518 + 2.0_f64 * t13440 * t1518 + 4.0_f64 * t2322 * t4292 + 2.0_f64 * t2371 * t4248 + 4.0_f64 * t4292 * t5523 + t13425 + 2.0_f64 * t13429;
    t13517
}
