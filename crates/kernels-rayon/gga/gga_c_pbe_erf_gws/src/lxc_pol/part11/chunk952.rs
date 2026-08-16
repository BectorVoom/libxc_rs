//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 952/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk952(t1903: f64, t2650: f64, t2654: f64, t5441: f64, t723: f64, t7733: f64, t5434: f64, t2735: f64, t561: f64, t996: f64, t1022: f64, t7116: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22986 = t2650 * t1903;
    let t22988 = t2654 * t5441;
    let t22994 = t7733 * t723;
    let t22996 = t2654 * t5434;
    let t23109 = t561 * t2735 * t996;
    let t23123 = t7116 * t1022;
    (t22986, t22988, t22994, t22996, t23109, t23123)
}
