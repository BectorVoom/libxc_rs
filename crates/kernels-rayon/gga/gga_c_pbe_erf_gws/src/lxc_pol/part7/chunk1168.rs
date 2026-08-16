//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1168/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1168(t343: f64, t6385: f64, t20842: f64, t2135: f64, t2168: f64, t6698: f64, t8967: f64, t6316: f64, t6627: f64, t2189: f64, t6241: f64, t3139: f64, t6177: f64, t8903: f64) -> (f64, f64, f64, f64, f64) {
    let t20843 = t343 * t6385;
    let t20846 = t2168 * t20842 * t2135 * t20843;
    let t20847 = t8967 * t6698;
    let t20848 = 7.0_f64 / 6.0_f64 * t20847;
    let t20849 = t6627 * t6316;
    let t20851 = t6241 * t2189;
    let t20855 = 3.0_f64 / 8.0_f64 * t8903 * t3139 * t6177 * t20851;
    (t20846, t20848, t20849, t20851, t20855)
}
