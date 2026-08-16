//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1074/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1074(t12256: f64, t5819: f64, t12268: f64, t3367: f64, t5825: f64, t12327: f64, t6442: f64, t12331: f64, t300: f64, t6513: f64, t12485: f64, t6518: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20292 = t12256 * t5819;
    let t20297 = t12268 * t5819;
    let t20317 = t3367 * t5825;
    let t20356 = t12327 * t6442;
    let t20365 = t12331 * t6442;
    let t20400 = t300 * t6513;
    let t20472 = t12485 * t6518;
    (t20292, t20297, t20317, t20356, t20365, t20400, t20472)
}
