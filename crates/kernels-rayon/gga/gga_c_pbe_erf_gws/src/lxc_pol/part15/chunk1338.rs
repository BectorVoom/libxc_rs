//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1338/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1338(t2409: f64, t26668: f64, t3965: f64, t14001: f64, t14466: f64, t3959: f64, t9328: f64, t26655: f64, t14765: f64, t3074: f64, t4395: f64, t2362: f64) -> (f64, f64, f64, f64, f64) {
    let t54564 = t3965 * t2409 * t26668;
    let t54566 = t14001 * t14466;
    let t54567 = 7.0_f64 / 72.0_f64 * t54566;
    let t54572 = t3959 * t9328;
    let t54575 = t3965 * t2409 * t26655;
    let t54580 = t3074 * t4395 * t14765;
    let t54581 = t54580 * t2362;
    (t54564, t54567, t54572, t54575, t54581)
}
