//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1198/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1198(t133: f64, t19349: f64, t19351: f64, t2911: f64, t3637: f64, t3644: f64, t42825: f64, t42827: f64, t48747: f64, t48750: f64, t48760: f64, t48769: f64, t48771: f64, t48772: f64, t48777: f64, t48780: f64, t48787: f64, t48791: f64, t48795: f64, t8231: f64) -> f64 {
    let t48856 = t48747 - t19349 + t19351 + t48750 + t48760 - t48769 + t48771 + t48772 - t48777 - t48780 + 0.1034553e3_f64 * t133 * t48787 - 0.12414636e3_f64 * t2911 * t8231 * t3644 * t3637 + 0.15518295e2_f64 * t133 * t48791 - 0.1724255e1_f64 * t133 * t48795 + 0.22990066666666666667e1_f64 * t42825 + 0.2758808e2_f64 * t42827;
    t48856
}
