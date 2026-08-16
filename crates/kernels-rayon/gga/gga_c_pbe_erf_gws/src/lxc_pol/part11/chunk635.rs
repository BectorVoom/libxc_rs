//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 635/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk635(t39: f64, t5772: f64, t5773: f64, t505: f64, t96: f64, t1235: f64, t125: f64, t128: f64, t2: f64, t512: f64, t131: f64, t120: f64, t133: f64, t1365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5776 = 0.19486833333333333333e1_f64 * t5772 * t5773 * t39;
    let t5825 = 1.0_f64 / t505 / t96;
    let t5832 = t125 * t1235;
    let t5833 = t128 * t2;
    let t5836 = 0.32645333333333333334e0_f64 * t5832 * t5833 * t39;
    let t5852 = t512 * t512;
    let t5853 = 1.0_f64 / t5852;
    let t5854 = t131 * t5853;
    let t5863 = 0.89405814814814814813e0_f64 * t133 * t1365 * t120;
    (t5776, t5825, t5832, t5833, t5836, t5852, t5853, t5854, t5863)
}
