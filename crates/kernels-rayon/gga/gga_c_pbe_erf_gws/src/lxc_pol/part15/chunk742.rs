//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 742/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk742(t4656: f64, t4741: f64, t60: f64, t40: f64, t1422: f64, t460: f64, t1322: f64, t4605: f64, t4607: f64, t470: f64, t1336: f64, t461: f64) -> (f64, f64, f64, f64) {
    let t4742 = t4656 + t4741;
    let t4743 = t60 * t4742;
    let t4744 = t40 * t4743;
    let t4745 = t1422 * t460;
    let t4746 = t40 * t4745;
    let t4749 = t4605 * t4607 * t1322;
    let t4750 = t470 * t4749;
    let t4751 = 0.1038945353962551798e3_f64 * t4750;
    let t4753 = t1336 * t461;
    (t4744, t4746, t4751, t4753)
}
