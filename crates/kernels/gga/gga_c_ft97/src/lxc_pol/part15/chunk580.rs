//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 580/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk580<F: Float>(t1068: F, t8640: F, t171: F, t7741: F, t1075: F, t7773: F, t89: F, t998: F, t1008: F, t2057: F, t1018: F, t1636: F, t1026: F, t8232: F, t1045: F, t2178: F) -> (F, F, F, F, F, F, F, F) {
    let t12165 = t8640 * t1068;
    let t12168 = 1.0 / t171 / t7741;
    let t12204 = t8640 * t1075;
    let t12362 = t89 * t7773 * t998;
    let t12401 = t2057 * t1008;
    let t12571 = t89 * t1636 * t1018;
    let t12617 = t8232 * t1026;
    let t12664 = t1045 * t2178;
    (t12165, t12168, t12204, t12362, t12401, t12571, t12617, t12664)
}
