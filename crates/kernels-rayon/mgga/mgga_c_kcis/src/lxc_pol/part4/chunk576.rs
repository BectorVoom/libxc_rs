//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 576/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk576(t260: f64, t2939: f64, t2986: f64, t2917: f64, t2919: f64, t2922: f64, t2925: f64, t2928: f64, t274: f64, t45: f64, t956: f64, t270: f64, t961: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2987 = t260 * t260;
    let t2988 = 1.0_f64 / t2987;
    let t2989 = t2939 * t2988;
    let t2991 = 0.16081824322151104822e2_f64 * t2986 * t2989;
    let t2992 = 0.12361111111111111111e-1_f64 * t2917;
    let t2997 = t2992 + 0.61805555555555555556e-2_f64 * t2919 - 0.61805555555555555555e-2_f64 * t2922 + 0.18541666666666666667e-1_f64 * t2925 - 0.92708333333333333333e-2_f64 * t2928;
    let t2998 = t2997 * t274;
    let t3001 = t45 * t956;
    let t3004 = t961 * t270;
    (t2987, t2988, t2989, t2991, t2992, t2997, t2998, t3001, t3004)
}
