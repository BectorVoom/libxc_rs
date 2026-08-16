//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 728/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk728(t3772: f64, t817: f64, t2365: f64, t3747: f64, t1114: f64, t833: f64, t3889: f64, t840: f64, t4383: f64, t6158: f64, t328: f64, t3780: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11318 = t3772 * t817;
    let t11347 = t3747 * t2365;
    let t11348 = t1114 * t11347;
    let t11349 = t11348 * t833;
    let t11368 = t840 * t3889;
    let t11374 = t6158 * t4383;
    let t11375 = t1114 * t11374;
    let t11387 = t3780 * t328;
    (t11318, t11347, t11348, t11349, t11368, t11375, t11387)
}
