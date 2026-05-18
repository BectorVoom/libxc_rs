//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1165/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1165<F: Float>(t7580: F, t92174: F, t26597: F, t26623: F, t700: F, t9251: F, t2387: F, t26620: F, t7589: F, t209: F, t2403: F, t2389: F, t2404: F, t705: F) -> (F, F, F, F, F, F) {
    let t92175 = t7580 * t92174;
    let t92177 = t26597 * t26623;
    let t92179 = t9251 * t700;
    let t92181 = t26620 * t92179 * t2387;
    let t92182 = t7589 * t92181;
    let t92184 = t209 * t2403;
    let t92187 = t92184 * t2389 * t2404 * t705;
    (t92175, t92177, t92181, t92182, t92184, t92187)
}
