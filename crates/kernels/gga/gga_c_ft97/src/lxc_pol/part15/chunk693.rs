//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 693/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk693<F: Float>(t18001: F, t4951: F, t4950: F, t17836: F, t4948: F, t3780: F, t52: F) -> (F, F, F, F) {
    let t21134 = t4951 * t18001;
    let t21135 = t4950 * t21134;
    let t21144 = t17836 * t4948;
    let t21145 = t52 * t3780;
    (t21134, t21135, t21144, t21145)
}
