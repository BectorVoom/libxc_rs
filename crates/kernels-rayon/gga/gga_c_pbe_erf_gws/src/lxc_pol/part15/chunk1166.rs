//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1166/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1166(t14432: f64, t14477: f64, t14620: f64, t14660: f64, t14703: f64, t14739: f64, t14775: f64, t14814: f64, t2053: f64, t4188: f64, t944: f64, t1167: f64, t810: f64) -> (f64, f64, f64, f64) {
    let t14817 = t14432 + t14477 + t14620 + t14660 + t14703 + t14739 + t14775 + t14814;
    let t14821 = t4188 * t2053;
    let t14822 = t14821 * t944;
    let t14825 = t1167 * t810;
    (t14817, t14821, t14822, t14825)
}
