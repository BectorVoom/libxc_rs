//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 837/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk837<F: Float>(t11262: F, t1526: F, t19950: F, t19965: F, t342: F, t630: F, t19961: F, t7705: F, t19957: F, t21048: F, t8675: F, t21025: F, t358: F, t21052: F, t21056: F, t21075: F) -> (F, F, F, F, F, F, F, F, F) {
    let t75881 = t1526 * t11262 * t19950;
    let t75935 = t342 * t630 * t19965;
    let t75944 = t1526 * t7705 * t19961;
    let t75947 = t1526 * t7705 * t19957;
    let t75994 = t8675 * t21048;
    let t75996 = t21025 * t358;
    let t76056 = t8675 * t21052;
    let t76062 = t8675 * t21056;
    let t76101 = t8675 * t21075;
    (t75881, t75935, t75944, t75947, t75994, t75996, t76056, t76062, t76101)
}
