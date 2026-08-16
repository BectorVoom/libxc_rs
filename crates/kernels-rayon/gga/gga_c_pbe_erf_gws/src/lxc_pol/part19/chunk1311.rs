//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1311/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1311(t14028: f64, t3863: f64, t3816: f64, t51371: f64, t1125: f64, t54101: f64, t11991: f64, t14011: f64, t54023: f64, t3754: f64, t51255: f64, t14570: f64, t9108: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56988 = t14028 * t3863;
    let t56990 = t51371 * t3816;
    let t56992 = t1125 * t54101;
    let t56994 = t14011 * t11991;
    let t56998 = t1125 * t54023;
    let t57000 = t51255 * t3754;
    let t57002 = t9108 * t14570;
    (t56988, t56990, t56992, t56994, t56998, t57000, t57002)
}
