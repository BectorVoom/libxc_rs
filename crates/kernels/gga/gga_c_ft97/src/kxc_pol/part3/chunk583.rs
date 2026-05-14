//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 583/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk583<F: Float>(t1526: F, t1529: F, t7705: F, t1533: F, t342: F, t630: F, t360: F, t18: F, t23: F, t7241: F, t174: F, t358: F, t1556: F, t357: F, t1636: F, t355: F) -> (F, F, F, F, F, F, F, F) {
    let t7707 = t1526 * t7705 * t1529;
    let t7710 = t342 * t630 * t1533;
    let t7741 = t360 * t360;
    let t7742 = 1.0 / t7741;
    let t7743 = t18 * t7742;
    let t7750 = t23 * t7241;
    let t7760 = 1.0 / t174 / t358;
    let t7763 = 1.0 / t1556 / t357;
    let t7773 = t1636 * t355;
    (t7707, t7710, t7741, t7743, t7750, t7760, t7763, t7773)
}
