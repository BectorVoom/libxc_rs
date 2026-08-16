//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 761/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk761(t465: f64, t4813: f64, t1425: f64, t409: f64, t414: f64, t1333: f64, t461: f64, t1438: f64, t428: f64, t1319: f64, t456: f64, t4607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4814 = t465 * t4813;
    let t4815 = 0.56969282336565386482e-3_f64 * t4814;
    let t4819 = t409 * t1425;
    let t4821 = t414 * t1425;
    let t4825 = t1333 * t461;
    let t4826 = 60.0_f64 * t4825;
    let t4827 = t1438 * t428;
    let t4830 = t1333 * t428;
    let t4835 = t1319 * t4607 * t456;
    (t4815, t4819, t4821, t4826, t4827, t4830, t4835)
}
