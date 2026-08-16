//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1270/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1270(t3928: f64, t944: f64, t3717: f64, t1172: f64, t810: f64, t14767: f64, t2503: f64, t29260: f64, t3808: f64, t3972: f64, t3975: f64, t45096: f64, t51555: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56042 = t3928 * t944;
    let t56046 = t3717 * t944;
    let t56053 = t1172 * t810;
    let t56061 = t14767 * t2503;
    let t56067 = t3972 * t3975 * t3808 * t29260;
    let t56070 = t51555 * t3975 * t45096;
    (t56042, t56046, t56053, t56061, t56067, t56070)
}
