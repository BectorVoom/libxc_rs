//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 439/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk439(t278: f64, t1646: f64, t994: f64, t993: f64, t1697: f64) -> (f64, f64, f64) {
    let t288 = 0.0_f64 < t278;
    let t1699 = t994 * t1646;
    let t1700 = t993 * t1699;
    let t1704 = piecewise3(t288, t1697, -t1697);
    (t1699, t1700, t1704)
}
