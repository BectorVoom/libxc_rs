//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1051/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1051(t1343: f64, t1383: f64, t169: f64, t5718: f64, t700: f64, t1355: f64, t4598: f64, t770: f64, t413: f64, t745: f64, t16447: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19013 = t169 * t1343 * t1383;
    let t19020 = t169 * t5718 * t700;
    let t19023 = t169 * t1355 * t1383;
    let t19026 = t169 * t770 * t4598;
    let t19028 = t413 * t745;
    let t19031 = t169 * t16447 * t242;
    (t19013, t19020, t19023, t19026, t19028, t19031)
}
