//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 889/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk889(t16895: f64, t1827: f64, t4967: f64, t587: f64, t610: f64, t1627: f64, t5149: f64, t16874: f64, t16876: f64, t16877: f64, t16881: f64, t16884: f64, t16889: f64, t16891: f64, t16893: f64) -> (f64, f64, f64, f64) {
    let t16896 = 64.0_f64 / 45.0_f64 * t16895;
    let t16900 = 32.0_f64 / 15.0_f64 * t587 * t1827 * t4967 * t610;
    let t16902 = 32.0_f64 / 15.0_f64 * t1627 * t5149;
    let t16903 = -t16874 - t16876 + 0.44134814814814814813e-2_f64 * t16877 - t16881 - t16884 + t16889 - t16891 - t16893 + t16896 - t16900 - t16902;
    (t16896, t16900, t16902, t16903)
}
