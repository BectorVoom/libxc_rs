//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 829/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk829(t4456: f64, t6123: f64, t6754: f64, t6835: f64, t945: f64, t321: f64, t2054: f64, t804: f64, t810: f64, t2182: f64, t2429: f64, t946: f64) -> (f64, f64, f64, f64, f64) {
    let t6837 = t4456 + t6123 + t6754 + t6835;
    let t6838 = t6837 * t945;
    let t6839 = t321 * t6838;
    let t6841 = t804 * t2054 * t810;
    let t6845 = t2429 * t946 * t2182;
    (t6837, t6838, t6839, t6841, t6845)
}
