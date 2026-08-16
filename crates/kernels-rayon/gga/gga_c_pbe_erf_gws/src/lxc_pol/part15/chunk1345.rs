//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1345/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1345(t13792: f64, t8695: f64, t13939: f64, t3083: f64, t2409: f64, t26933: f64, t3959: f64, t13953: f64, t3070: f64, t4141: f64, t50998: f64, t9521: f64) -> (f64, f64, f64, f64, f64) {
    let t54664 = t13792 * t8695;
    let t54667 = 7.0_f64 / 144.0_f64 * t3083 * t13939;
    let t54675 = t3959 * t2409 * t26933;
    let t54681 = t13953 * t3070;
    let t54682 = 7.0_f64 / 72.0_f64 * t54681;
    let t54690 = t50998 * t4141 * t9521;
    (t54664, t54667, t54675, t54682, t54690)
}
