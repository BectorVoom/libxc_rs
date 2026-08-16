//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 558/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk558(t260: f64, t2917: f64, t45: f64, t956: f64, t270: f64, t961: f64) -> (f64, f64, f64, f64, f64) {
    let t2987 = t260 * t260;
    let t2988 = 1.0_f64 / t2987;
    let t2992 = 0.12361111111111111111e-1_f64 * t2917;
    let t3001 = t45 * t956;
    let t3004 = t961 * t270;
    let t3005 = 1.0_f64 / t3004;
    (t2987, t2988, t2992, t3001, t3005)
}
