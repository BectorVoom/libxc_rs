//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 804/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk804(t651: f64, t8641: f64, t1955: f64, t2061: f64, t251: f64, t3140: f64) -> (f64, f64, f64) {
    let t8643 = 2.0_f64 * t651 * t8641;
    let t8645 = t1955 * t2061;
    let t8648 = t251 * t3140;
    (t8643, t8645, t8648)
}
