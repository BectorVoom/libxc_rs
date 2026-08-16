//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1931/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1931(t3936: f64, t4004: f64, t5704: f64, t3924: f64, t2482: f64, t4000: f64, t814: f64) -> (f64, f64, f64) {
    let t13834 = t3936 * t5704 * t4004;
    let t13841 = t3936 * t5704 * t3924;
    let t13845 = t2482 * t4000 * t814;
    (t13834, t13841, t13845)
}
