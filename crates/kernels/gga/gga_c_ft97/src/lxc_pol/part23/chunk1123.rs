//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1123/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1123<F: Float>(t108581: F, t6034: F, t6037: F, t65692: F, t695: F, t24378: F, t27501: F, t27500: F, t1109: F, t24310: F, t6043: F, t6824: F, t96535: F, t3750: F, t3773: F, t6027: F) -> (F, F, F, F, F, F, F) {
    let t108583 = t6034 * t108581 * t6037;
    let t108585 = t65692 * t695;
    let t108596 = t24378 * t27501;
    let t108597 = t27500 * t108596;
    let t108606 = t24310 * t1109;
    let t108618 = t6043 * t96535 * t6824;
    let t108629 = t3773 * t6027 * t3750;
    (t108583, t108585, t108596, t108597, t108606, t108618, t108629)
}
