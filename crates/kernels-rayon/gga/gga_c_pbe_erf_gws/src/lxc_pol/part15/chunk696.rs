//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 696/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk696(t3990: f64, t3991: f64, t875: f64, t3989: f64, t1178: f64, t371: f64, t939: f64, t1177: f64, t1192: f64, t2376: f64, t830: f64, t829: f64) -> (f64, f64, f64, f64, f64) {
    let t3993 = t3990 * t3991 * t875;
    let t3994 = t3989 * t3993;
    let t3997 = t371 * t1178 * t939;
    let t3998 = t1177 * t3997;
    let t4000 = t2376 * t1192;
    let t4001 = t830 * t4000;
    let t4002 = t829 * t4001;
    (t3993, t3994, t3997, t3998, t4002)
}
