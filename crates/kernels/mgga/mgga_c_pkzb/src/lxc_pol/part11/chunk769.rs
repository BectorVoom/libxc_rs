//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 769/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk769<F: Float>(t7930: F, t179: F, t2405: F, t3026: F, t404: F, t1227: F, t931: F, t300: F) -> (F, F, F, F, F, F) {
    let t8225 = 0.34246666666666666666e-1 * t7930;
    let t8233 = 0.35616666666666666666e-1 * t7930;
    let t8245 = t179 * t2405 * t3026;
    let t8247 = 0.57165357490759649296e-3 * t404 * t8245;
    let t8253 = t931 * t1227;
    let t8254 = t300 * t8253;
    (t8225, t8233, t8245, t8247, t8253, t8254)
}
