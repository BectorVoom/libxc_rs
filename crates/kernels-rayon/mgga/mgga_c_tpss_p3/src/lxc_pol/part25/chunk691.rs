//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 691/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk691(t1613: f64, t724: f64, t489: f64, t2281: f64, t2285: f64, t3182: f64, t3189: f64, t3194: f64, t3196: f64, t4357: f64, t4359: f64, t4379: f64, t4428: f64, t4429: f64, t4431: f64) -> (f64, f64, f64) {
    let t4432 = t1613 * t724;
    let t4433 = t489 * t4432;
    let t4434 = t4357 - t4359 + t4379 - t4428 - t3182 - t2285 - t2281 + t3189 - t4429 + t3194 - t3196 + t4431 + t4433;
    (t4432, t4433, t4434)
}
