//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1171/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1171(t2156: f64, t816: f64, t2157: f64, t2074: f64, t824: f64, t821: f64, t6184: f64, t6217: f64, t6538: f64, t2313: f64, t745: f64, t2148: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20886 = t816 * t2156;
    let t20887 = t20886 * t2157;
    let t20898 = t824 * t2074;
    let t20899 = t821 * t20898;
    let t20903 = t6217 * t6184;
    let t20904 = 7.0_f64 / 24.0_f64 * t20903;
    let t20905 = t6538 * t6184;
    let t20906 = 7.0_f64 / 24.0_f64 * t20905;
    let t20907 = t2313 * t745;
    let t20912 = t2148 * t2074;
    (t20886, t20887, t20899, t20904, t20906, t20907, t20912)
}
