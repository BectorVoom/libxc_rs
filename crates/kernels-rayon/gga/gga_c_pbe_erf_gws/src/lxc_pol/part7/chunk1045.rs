//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1045/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1045(t16576: f64, t88: f64, t18853: f64, t18863: f64, t18928: f64, t18933: f64, t18935: f64, t18939: f64, t18941: f64, t18944: f64, t18946: f64, t18950: f64, t18954: f64) -> (f64, f64) {
    let t18955 = t16576 * t88;
    let t18956 = 384.0_f64 * t18955;
    let t18957 = t18853 - t18863 + t18928 - t18933 + t18935 + t18939 + t18941 + t18944 + t18946 - t18950 + t18954 - t18956;
    (t18956, t18957)
}
