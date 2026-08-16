//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 830/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk830(t2424: f64, t804: f64, t810: f64, t2051: f64, t944: f64, t2052: f64, t381: f64, t321: f64, t2074: f64, t946: f64, t2075: f64, t2429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6850 = t804 * t2424 * t810;
    let t6852 = t2051 * t944;
    let t6854 = 1.0_f64 / t2052 / t381;
    let t6855 = t6852 * t6854;
    let t6856 = t321 * t6855;
    let t6860 = t804 * t946 * t2074;
    let t6863 = t2429 * t2075 * t810;
    (t6850, t6854, t6855, t6856, t6860, t6863)
}
