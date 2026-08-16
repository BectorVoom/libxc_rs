//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 449/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk449(t242: f64, t2519: f64, t700: f64, t992: f64, t1072: f64, t168: f64, t703: f64, t1069: f64, t735: f64, t92: f64, t950: f64, t93: f64, t954: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2520 = t2519 * t242;
    let t2526 = t992 * t700;
    let t2531 = t168 * t703 * t1072;
    let t2536 = t1069 * t735;
    let t2538 = t92 * t950;
    let t2544 = t93 * t954;
    (t2520, t2526, t2531, t2536, t2538, t2544)
}
