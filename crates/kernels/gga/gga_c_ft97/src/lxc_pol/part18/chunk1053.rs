//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1053/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1053<F: Float>(t1586: F, t22: F, t36452: F, t37991: F, t96: F, t1554: F, t355: F, t7241: F, t369: F, t7760: F, t102: F, t8416: F, t100: F, t480: F, t8417: F, t1841: F, t1851: F) -> (F, F, F, F, F, F, F, F) {
    let t38456 = 1.0 / t96 / t37991 / t22 / t1586 / t36452 / 96.0;
    let t38463 = t1554 * t1586;
    let t38477 = t355 * t7241;
    let t38482 = t7760 * t369;
    let t38651 = 1.0 / t8416 / t102;
    let t38652 = t100 * t38651;
    let t38659 = t480 * t8417;
    let t38664 = t1841 * t1851;
    (t38456, t38463, t38477, t38482, t38651, t38652, t38659, t38664)
}
