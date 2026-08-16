//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 854/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk854(t1873: f64, t3532: f64, t667: f64, t672: f64, t9164: f64, t218: f64, t3542: f64, t675: f64) -> (f64, f64, f64) {
    let t9177 = t1873 * t3532;
    let t9178 = t9177 * t667;
    let t9180 = t672 * t9164;
    let t9185 = t218 * t675 * t3542;
    (t9178, t9180, t9185)
}
