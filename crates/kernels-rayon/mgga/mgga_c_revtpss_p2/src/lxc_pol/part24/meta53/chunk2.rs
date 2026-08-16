//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 358/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk358(t1340: f64, t762: f64, t531: f64, t566: f64, t513: f64, t516: f64, t212: f64, t555: f64) -> (f64, f64, f64, f64, f64) {
    let t1342 = 0.5848223622634646207e0_f64 * t1340 * t762;
    let t1343 = t531 * t566;
    let t1344 = 1.0_f64 / t513;
    let t1348 = 1.0_f64 / t516;
    let t1357 = t212 * t555;
    (t1342, t1343, t1344, t1348, t1357)
}
