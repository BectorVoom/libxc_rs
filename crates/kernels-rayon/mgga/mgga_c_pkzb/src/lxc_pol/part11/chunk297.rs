//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 297/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk297(t1024: f64, t581: f64, t1010: f64, t1012: f64, t158: f64, t469: f64, t494: f64, t498: f64, t503: f64, t547: f64, t554: f64, t559: f64) -> (f64, f64) {
    let t1025 = t581 * t1024;
    let t1029 = (t469 + t494 - t498 - t503 + t1010 + t547 + t1012 - t554 - t559) * t158;
    (t1025, t1029)
}
