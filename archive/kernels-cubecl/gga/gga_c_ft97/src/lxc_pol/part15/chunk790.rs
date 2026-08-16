//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 790/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk790<F: Float>(t192: F, t21416: F, t9942: F, t2372: F, t3930: F, t5053: F, t21181: F, t9953: F, t9952: F, t2487: F, t737: F, t21204: F, t3917: F) -> (F, F, F, F, F, F, F) {
    let t21577 = t192 * t9942 * t21416;
    let t21581 = t2372 * t3930 * t5053;
    let t21584 = t9953 * t21181;
    let t21585 = t9952 * t21584;
    let t21588 = t2487 * t21181;
    let t21589 = t737 * t21588;
    let t21592 = t3917 * t21204;
    (t21577, t21581, t21584, t21585, t21588, t21589, t21592)
}
