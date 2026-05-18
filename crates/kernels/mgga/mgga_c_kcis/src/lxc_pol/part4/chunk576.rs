//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 576/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk576<F: Float>(t260: F, t2939: F, t2986: F, t2917: F, t2919: F, t2922: F, t2925: F, t2928: F, t274: F, t45: F, t956: F, t270: F, t961: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2987 = t260 * t260;
    let t2988 = F::new(1.0) / t2987;
    let t2989 = t2939 * t2988;
    let t2991 = F::new(0.16081824322151104822e2) * t2986 * t2989;
    let t2992 = F::new(0.12361111111111111111e-1) * t2917;
    let t2997 = t2992 + F::new(0.61805555555555555556e-2) * t2919 - F::new(0.61805555555555555555e-2) * t2922 + F::new(0.18541666666666666667e-1) * t2925 - F::new(0.92708333333333333333e-2) * t2928;
    let t2998 = t2997 * t274;
    let t3001 = t45 * t956;
    let t3004 = t961 * t270;
    (t2987, t2988, t2989, t2991, t2992, t2997, t2998, t3001, t3004)
}
