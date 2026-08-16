//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 407/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk407(t1691: f64, t1692: f64, t11: f64, t261: f64, t50: f64) -> (f64, f64, f64, f64) {
    let t1693 = t1691 * t1692;
    let t1694 = t11 * t1693;
    let t1696 = t261 * t50;
    let t1697 = 1.0_f64 / t1696;
    (t1693, t1694, t1696, t1697)
}
