//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 548/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk548(t1022: f64, t2829: f64, t1021: f64, t1020: f64, t299: f64, t977: f64, t278: f64) -> (f64, f64, f64, f64, f64) {
    let t2830 = t1022 * t2829;
    let t2831 = t1021 * t2830;
    let t2832 = t1020 * t2831;
    let t2835 = 1.0_f64 / t977 / t299;
    let t2836 = t278 * t2835;
    (t2830, t2831, t2832, t2835, t2836)
}
