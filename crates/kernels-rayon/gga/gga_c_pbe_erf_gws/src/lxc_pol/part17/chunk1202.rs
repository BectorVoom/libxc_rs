//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1202/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1202(t13808: f64, t14132: f64, t1176: f64, t2332: f64, t931: f64, t3985: f64, t13923: f64, t859: f64, t892: f64, t2079: f64, t376: f64, t14797: f64, t3973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51526 = t13808 * t14132;
    let t51529 = t1176 * t2332 * t931;
    let t51530 = t51529 * t3985;
    let t51540 = t859 * t892 * t13923;
    let t51543 = t376 * t2079;
    let t51548 = t3973 * t14797;
    (t51526, t51529, t51530, t51540, t51543, t51548)
}
