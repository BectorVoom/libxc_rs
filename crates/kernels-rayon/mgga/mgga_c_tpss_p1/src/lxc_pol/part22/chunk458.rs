//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 458/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk458(t1668: f64, t1670: f64, t547: f64, t548: f64, t38: f64, t578: f64) -> (f64, f64) {
    let t1673 = t1668 * t548 + 3.0_f64 * t1670 * t547;
    let t1675 = t578 * t38;
    (t1673, t1675)
}
