//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 819/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk819(t2074: f64, t938: f64, t2376: f64, t2409: f64, t2182: f64, t2383: f64, t3074: f64, t2112: f64, t829: f64, t830: f64, t831: f64, t2358: f64, t2382: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6755 = t2074 * t938;
    let t6757 = t2409 * t2376 * t6755;
    let t6760 = t2182 * t938;
    let t6762 = t2409 * t2376 * t6760;
    let t6769 = t3074 * t2383;
    let t6772 = t829 * t830 * t831 * t2112;
    let t6775 = t2382 * t2358;
    (t6755, t6757, t6760, t6762, t6769, t6772, t6775)
}
