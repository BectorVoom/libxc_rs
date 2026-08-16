//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 500/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk500(t1573: f64, t324: f64, t1541: f64, t1548: f64, t1551: f64, t1554: f64, t945: f64, t948: f64) -> (f64, f64) {
    let t1574 = t1573 * t324;
    let t1580 = 0.258925e1_f64 * t1548 - t945 - 0.301925e0_f64 * t1541 + 0.16504875e0_f64 * t1551 - t948 - 0.82785e-1_f64 * t1554;
    (t1574, t1580)
}
