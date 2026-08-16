//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2358/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2358<F: Float>(t13555: F, t13784: F, t2986: F, t13528: F, t1592: F, t42891: F, t973: F, t13812: F, t13822: F, t13881: F, t13886: F, t10263: F, t4506: F) -> (F, F, F, F, F, F, F) {
    let t48390 = t2986 * t13784 * t13555;
    let t48394 = t2986 * t13784 * t13528;
    let t48397 = t973 * t42891 * t1592;
    let t48402 = t973 * t13822 * t13812;
    let t48407 = t973 * t13822 * t13881;
    let t48417 = t973 * t13822 * t13886;
    let t48421 = t10263 * t4506;
    (t48390, t48394, t48397, t48402, t48407, t48417, t48421)
}
