//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1289/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1289(t15309: f64, t51963: f64, t4127: f64, t8751: f64, t13972: f64, t15169: f64, t1123: f64, t2848: f64, t331: f64, t833: f64, t850: f64, t11576: f64, t14423: f64, t14682: f64, t3989: f64) -> (f64, f64, f64, f64, f64) {
    let t56553 = t51963 * t15309;
    let t56555 = t4127 * t8751;
    let t56560 = t13972 * t15169;
    let t56578 = t850 * t1123 * t2848 * t331 * t833;
    let t56582 = t3989 * t14682 * t14423 * t11576;
    (t56553, t56555, t56560, t56578, t56582)
}
