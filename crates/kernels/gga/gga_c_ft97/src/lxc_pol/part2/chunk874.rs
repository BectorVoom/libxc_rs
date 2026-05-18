//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 874/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk874<F: Float>(t13617: F, t701: F, t13593: F, t13596: F, t13601: F, t13603: F, t13607: F, t13611: F, t13614: F, t9639: F, t9642: F, t9648: F) -> (F, F) {
    let t13618 = t701 * t13617;
    let t13623 = F::new(0.38306165027777777778e-1) * t13593 - F::new(0.85124811172839506173e-2) * t13596 + t13601 + F::new(0.85124811172839506173e-2) * t13603 + F::new(0.19862455940329218107e-1) * t13607 - F::new(0.3404992446913580247e-1) * t13611 - F::new(0.12768721675925925926e-1) * t13614 + F::new(0.51074886703703703704e-1) * t13618 - F::new(0.28374937057613168724e-2) * t9639 + F::new(0.21281202793209876543e-2) * t9648 + F::new(0.28374937057613168724e-2) * t9642;
    (t13618, t13623)
}
