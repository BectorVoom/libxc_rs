//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1293/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1293(t13781: f64, t15144: f64, t3038: f64, t3972: f64, t14696: f64, t39061: f64, t3975: f64, t38036: f64, t6472: f64, t820: f64, t14767: f64, t3047: f64) -> (f64, f64, f64, f64) {
    let t56651 = t3972 * t13781 * t3038 * t15144;
    let t56657 = t3972 * t3975 * t39061 * t14696;
    let t56667 = t3972 * t3975 * t38036 * t6472 * t820;
    let t56674 = t14767 * t3047;
    (t56651, t56657, t56667, t56674)
}
