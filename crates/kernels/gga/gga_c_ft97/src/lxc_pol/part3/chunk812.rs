//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 812/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk812<F: Float>(t1526: F, t4641: F, t7705: F, t142: F, t8633: F, t2984: F, t2258: F, t2993: F, t18: F, t1943: F, t342: F, t4645: F, t630: F) -> (F, F, F, F, F) {
    let t16631 = t1526 * t7705 * t4641;
    let t16633 = t8633 * t142;
    let t16634 = t16633 * t2984;
    let t16640 = t2258 * t142;
    let t16641 = t16640 * t2993;
    let t16644 = t1943 * t18;
    let t16649 = t342 * t630 * t4645;
    (t16631, t16634, t16641, t16644, t16649)
}
