//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 661/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk661<F: Float>(t2253: F, t5450: F, t5454: F, t10904: F, t5457: F, t1526: F, t5198: F, t9483: F, t10915: F, t294: F, t2917: F, t342: F, t5202: F, t630: F) -> (F, F, F, F, F, F, F) {
    let t18900 = t2253 * t5450;
    let t18902 = t2253 * t5454;
    let t18926 = t10904 * t5457;
    let t18959 = t1526 * t9483 * t5198;
    let t18961 = t10915 * t294;
    let t18968 = t2917 * t294;
    let t18977 = t342 * t630 * t5202;
    (t18900, t18902, t18926, t18959, t18961, t18968, t18977)
}
