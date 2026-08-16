//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 549/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk549(t3703: f64, t376: f64, t353: f64, t338: f64, t1105: f64, t1161: f64, t2376: f64, t2409: f64, t1076: f64, t823: f64) -> (f64, f64, f64, f64, f64) {
    let t3737 = t376 * t3703;
    let t3738 = t353 * t3737;
    let t3739 = t338 * t3738;
    let t3742 = t1105 * t1161;
    let t3744 = t2409 * t2376 * t3742;
    let t3747 = t823 * t1076;
    (t3737, t3739, t3742, t3744, t3747)
}
