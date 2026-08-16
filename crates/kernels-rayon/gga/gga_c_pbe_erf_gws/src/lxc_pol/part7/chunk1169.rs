//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1169/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1169(t6310: f64, t6627: f64, t6484: f64, t6530: f64, t20296: f64, t2168: f64, t2170: f64, t2171: f64, t20264: f64, t20832: f64, t20837: f64, t20840: f64, t20846: f64, t20848: f64, t20849: f64, t20855: f64, t3140: f64, t3235: f64, t3247: f64) -> (f64, f64, f64) {
    let t20856 = t6627 * t6310;
    let t20858 = t6484 * t6530;
    let t20859 = 7.0_f64 / 12.0_f64 * t20858;
    let t20863 = t2168 * t2170 * t20296 * t2171 / 12.0_f64;
    let t20868 = t20832 - t20837 + 119.0_f64 / 144.0_f64 * t20840 + t20846 + t20848 - 7.0_f64 / 48.0_f64 * t20849 - t20855 + 7.0_f64 / 192.0_f64 * t20856 - t20859 + t20863 + t3247 * t3235 * t20264 * t3140 / 128.0_f64;
    (t20859, t20863, t20868)
}
