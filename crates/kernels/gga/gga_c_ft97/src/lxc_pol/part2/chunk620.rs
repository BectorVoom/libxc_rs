//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 620/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk620<F: Float>(t213: F, t51: F, t1109: F, t6: F, t694: F, t373: F, t929: F, t237: F, t173: F, t174: F, t368: F, t2: F) -> (F, F, F, F, F, F, F) {
    let t4951 = t51 * t213;
    let t4952 = t4951 * t1109;
    let t6032 = t694 * t6;
    let t6426 = t373 * t929;
    let t6783 = t237 * t6;
    let t7239 = t173 * t174;
    let t7240 = t368 * t368;
    let t7241 = F::new(1.0) / t7240;
    let t7242 = t2 * t2;
    (t4952, t6032, t6426, t6783, t7239, t7241, t7242)
}
