//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 675/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk675(t5406: f64, t593: f64, t1648: f64, t1656: f64, t1666: f64, t1651: f64, t1655: f64, t587: f64, t1923: f64, t707: f64, t256: f64, t1914: f64, t1918: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5408 = 4.0_f64 / 15.0_f64 * t5406 * t593;
    let t5410 = 4.0_f64 / 15.0_f64 * t1648 * t1656;
    let t5412 = 4.0_f64 / 9.0_f64 * t1648 * t1666;
    let t5413 = t1651 * t1655;
    let t5414 = t587 * t5413;
    let t5415 = 8.0_f64 / 45.0_f64 * t5414;
    let t5416 = t707 * t1923;
    let t5417 = t5416 * t256;
    let t5418 = t1914 * t1918;
    (t5408, t5410, t5412, t5413, t5415, t5416, t5417, t5418)
}
