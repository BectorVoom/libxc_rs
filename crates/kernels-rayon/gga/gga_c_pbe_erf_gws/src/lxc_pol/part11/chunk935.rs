//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 935/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk935(t2209: f64, t2365: f64, t6658: f64, t825: f64, t19562: f64, t346: f64, t6274: f64, t6684: f64, t6553: f64, t899: f64, t922: f64, t6587: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20550 = t2365 * t2209;
    let t20560 = t825 * t6658;
    let t20585 = t19562 * t346;
    let t20607 = t6684 * t6274;
    let t20625 = t899 * t6553 * t922;
    let t20646 = t899 * t912 * t6587;
    (t20550, t20560, t20585, t20607, t20625, t20646)
}
