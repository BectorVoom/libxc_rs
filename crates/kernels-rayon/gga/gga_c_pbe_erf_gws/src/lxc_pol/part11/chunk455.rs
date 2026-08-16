//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 455/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk455(t2654: f64, t714: f64, t1062: f64, t723: f64, t1697: f64, t954: f64, t1640: f64, t219: f64) -> (f64, f64, f64, f64) {
    let t2655 = t2654 * t714;
    let t2657 = t1062 * t723;
    let t2672 = t1697 * t954;
    let t2677 = t1640 * t219;
    (t2655, t2657, t2672, t2677)
}
