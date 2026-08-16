//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 313/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk313(t1419: f64, t225: f64, t561: f64, t213: f64, t555: f64, t560: f64) -> (f64, f64, f64, f64, f64) {
    let t1420 = t1419 * t225;
    let t1421 = t1420 * t561;
    let t1424 = t213 * t555;
    let t1425 = t560 * t560;
    let t1426 = 1.0_f64 / t1425;
    (t1420, t1421, t1424, t1425, t1426)
}
