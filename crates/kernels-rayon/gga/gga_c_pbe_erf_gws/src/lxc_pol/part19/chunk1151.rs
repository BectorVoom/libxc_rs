//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1151/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1151(t14765: f64, t2306: f64, t3074: f64, t833: f64, t2409: f64, t9716: f64, t3959: f64, t3298: f64, t3975: f64, t3972: f64, t9707: f64, t3965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14766 = t2306 * t14765;
    let t14767 = t3074 * t14766;
    let t14768 = t14767 * t833;
    let t14772 = t2409 * t9716;
    let t14773 = t3959 * t14772;
    let t14776 = t3975 * t3298;
    let t14777 = t3972 * t14776;
    let t14781 = t2409 * t9707;
    let t14782 = t3965 * t14781;
    (t14767, t14768, t14772, t14773, t14776, t14777, t14781, t14782)
}
