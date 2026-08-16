//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1102/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1102(t237: f64, t5838: f64, t1977: f64, t1083: f64, t5776: f64, t1066: f64, t204: f64, t3981: f64) -> (f64, f64, f64, f64) {
    let t20637 = t237 * t5838;
    let t20671 = t237 * t1977;
    let t20683 = t5776 * t1083;
    let t20705 = t204 * t3981 * t1066;
    (t20637, t20671, t20683, t20705)
}
