//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 985/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk985(t16760: f64, t16765: f64, t16768: f64, t16771: f64, t16775: f64, t16777: f64, t16781: f64, t16787: f64, t16792: f64, t16796: f64, t16800: f64, t5385: f64, t708: f64) -> (f64, f64) {
    let t18191 = t16760 + t16765 - t16768 - t16771 - t16775 - t16777 - t16781 - t16787 + t16792 + t16796 + t16800;
    let t18192 = t708 * t5385;
    (t18191, t18192)
}
