//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1007/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1007(t4652: f64, t4664: f64, t4744: f64, t4746: f64, t4751: f64, t4784: f64, t4790: f64, t6076: f64, t7985: f64, t7987: f64, t7989: f64, t7991: f64, t7992: f64, t7994: f64, t7995: f64, t7997: f64, t7999: f64, t8000: f64, t8001: f64) -> f64 {
    let t9044 = -t7985 + t7987 - t7989 + t7991 + t7992 + t4744 + t4746 + t4751 + t4652 - t7994 - t7995 + t4664 - t6076 + t7997 + t7999 - t4784 - t8000 - t4790 - t8001;
    t9044
}
