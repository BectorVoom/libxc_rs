//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 993/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk993<F: Float>(t342: F, t344: F, t8639: F, t1533: F, t2252: F, t1586: F, t22: F, t36452: F, t37991: F, t96: F, t1554: F, t355: F, t7241: F, t369: F, t7760: F, t102: F, t8416: F) -> (F, F, F, F, F, F, F) {
    let t38355 = 5.0 / 54.0 * t342 * t8639 * t344;
    let t38369 = t342 * t2252 * t1533;
    let t38456 = 1.0 / t96 / t37991 / t22 / t1586 / t36452 / 96.0;
    let t38463 = t1554 * t1586;
    let t38477 = t355 * t7241;
    let t38482 = t7760 * t369;
    let t38651 = 1.0 / t8416 / t102;
    (t38355, t38369, t38456, t38463, t38477, t38482, t38651)
}
