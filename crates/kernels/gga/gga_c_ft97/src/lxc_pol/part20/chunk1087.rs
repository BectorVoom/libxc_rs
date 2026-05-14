//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1087/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1087<F: Float>(t108590: F, t27669: F, t35456: F, t6817: F, t92354: F, t24378: F, t27501: F, t27500: F, t13863: F, t6035: F, t9665: F, t1109: F, t24310: F, t6043: F, t6824: F, t96535: F) -> (F, F, F, F, F, F, F) {
    let t108591 = t108590 * t27669;
    let t108593 = t92354 * t6817 * t35456;
    let t108596 = t24378 * t27501;
    let t108597 = t27500 * t108596;
    let t108600 = t6035 * t9665 * t13863;
    let t108606 = t24310 * t1109;
    let t108618 = t6043 * t96535 * t6824;
    (t108591, t108593, t108596, t108597, t108600, t108606, t108618)
}
