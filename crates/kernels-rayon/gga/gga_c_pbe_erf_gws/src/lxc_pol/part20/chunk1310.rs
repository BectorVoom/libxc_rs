//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1310/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1310(t11396: f64, t13780: f64, t13859: f64, t3990: f64, t11732: f64, t3989: f64, t3991: f64, t15338: f64, t4414: f64, t11509: f64, t3950: f64, t833: f64, t850: f64) -> (f64, f64, f64, f64) {
    let t56757 = t13859 * t3990 * t13780 * t11396;
    let t56761 = t3989 * t3990 * t3991 * t11732;
    let t56769 = t4414 * t15338;
    let t56773 = t850 * t11509 * t3950 * t833;
    (t56757, t56761, t56769, t56773)
}
