//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 965/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk965(t2268: f64, t2440: f64, t3340: f64, t10223: f64, t894: f64, t10151: f64, t1063: f64, t2343: f64, t6519: f64, t8195: f64, t9189: f64, t2854: f64, t29975: f64, t6320: f64) -> (f64, f64, f64, f64, f64) {
    let t42610 = t2268 * t2440 * t3340;
    let t42613 = t2268 * t894 * t10223;
    let t42625 = t1063 * t2343 * t10151 * t6519;
    let t42629 = 0.19918504644973304719e0_f64 * t2268 * t9189 * t8195;
    let t42633 = 0.17073003981405689759e1_f64 * t2268 * t6320 * t2854 * t29975;
    (t42610, t42613, t42625, t42629, t42633)
}
