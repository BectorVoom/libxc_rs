//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 784/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk784(t6105: f64, t824: f64, t905: f64, t2079: f64, t2081: f64, t820: f64, t2105: f64, t4394: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64) {
    let t6465 = t6105 * t824;
    let t6466 = t905 * t6465;
    let t6469 = t2079 * param_a_c;
    let t6470 = t2081 * t820;
    let t6471 = t6469 * t6470;
    let t6472 = t4394 * t2105;
    (t6466, t6469, t6470, t6471, t6472)
}
