//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 656/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk656(t362: f64, t6552: f64, t366: f64, t899: f64, t2209: f64, t825: f64, t346: f64, t6158: f64, t2251: f64, t2299: f64, t2276: f64, t22: f64, t4258: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6553 = t6552 * t362;
    let t6555 = t899 * t6553 * t366;
    let t6560 = t825 * t2209;
    let t6566 = t6158 * t346;
    let t6578 = t2251 * t2299;
    let t6579 = t2276 * t6578;
    let t6587 = 1.0_f64 / t22 / t4258;
    (t6553, t6555, t6560, t6566, t6579, t6587)
}
