//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 788/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk788<F: Float>(t21540: F, t242: F, t1168: F, t18391: F, t14114: F, t18431: F, t18452: F, t18455: F, t18457: F, t18538: F, t18540: F, t18542: F, t18544: F, t21524: F, t21533: F, t21537: F, t446: F, t9982: F) -> (F, F, F, F) {
    let t21541 = t242 * t21540;
    let t21548 = t18391 * t1168;
    let t21549 = t242 * t21548;
    let t21551 = F::new(2.0) / F::new(3.0) * t446 * t21524 + t18431 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t18452 - F::new(2.0) / F::new(9.0) * t18455 - F::new(2.0) / F::new(9.0) * t18457 - F::new(2.0) * t446 * t21533 - F::new(2.0) * t446 * t21537 - t446 * t21541 + F::new(4.0) / F::new(9.0) * t14114 - t9982 + F::new(2.0) / F::new(27.0) * t18538 + t18540 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t18542 - F::new(2.0) / F::new(3.0) * t18544 - t446 * t21549;
    (t21541, t21548, t21549, t21551)
}
