//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 687/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk687(t1114: f64, t6159: f64, t6154: f64, t1105: f64, t898: f64, t4423: f64, t833: f64, t1161: f64, t2416: f64, t6792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8659 = t1114 * t6159;
    let t8662 = t1114 * t6154;
    let t8713 = t898 * t1105;
    let t8746 = t1114 * t4423;
    let t8747 = t8746 * t833;
    let t8787 = t2416 * t1161;
    let t8793 = t1114 * t6792;
    (t8659, t8662, t8713, t8746, t8747, t8787, t8793)
}
