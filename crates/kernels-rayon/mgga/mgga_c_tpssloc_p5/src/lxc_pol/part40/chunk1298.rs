//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1298/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1298(t111457: f64, t111503: f64, t111546: f64, t111592: f64, t110489: f64, t110882: f64, t110884: f64, t110886: f64, t110888: f64, t111316: f64, t111317: f64, t111322: f64, t1404: f64, t1852: f64, t20186: f64, t2187: f64, t3: f64, t30263: f64, t30466: f64, t580: f64, t6483: f64, t8154: f64) -> (f64, f64) {
    let t111594 = t111457 + t111503 + t111546 + t111592;
    let t111597 = t111594 * t3 * t580 + t1404 * t30466 + 2.0_f64 * t1852 * t30263 + t20186 * t2187 + t6483 * t8154 + t110489 + t110882 + t110884 + t110886 + t110888 + t111316 + 2.0_f64 * t111317 + t111322;
    (t111594, t111597)
}
