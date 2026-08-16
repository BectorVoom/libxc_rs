//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 945/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk945(t4101: f64, t673: f64, t1515: f64, t2202: f64, t4048: f64, t664: f64) -> (f64, f64, f64, f64) {
    let t11844 = t673 * t4101;
    let t11845 = 0.10954222222222222222e0_f64 * t11844;
    let t11850 = t2202 * t1515;
    let t11873 = t664 * t4048;
    (t11844, t11845, t11850, t11873)
}
