//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 857/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk857(t6193: f64, t852: f64, t833: f64, t2238: f64, t831: f64, t338: f64) -> (f64, f64, f64, f64) {
    let t6194 = t6193 * t852;
    let t6196 = 1.0_f64 * t833 * t6194;
    let t6198 = 1.0_f64 / t2238 / t831;
    let t6199 = t338 * t6198;
    (t6194, t6196, t6198, t6199)
}
