//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 565/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk565<F: Float>(t7954: F, t82: F, t177: F, t2280: F, t1736: F, t70: F, t179: F, t7763: F, t11: F, t2247: F) -> (F, F, F, F, F) {
    let t8577 = t7954 * t82;
    let t8618 = F::cast_from(1.0_f64) / t2280 / t177;
    let t8633 = t70 * t1736;
    let t8634 = t179 * t7763;
    let t8639 = t11 * t2247;
    (t8577, t8618, t8633, t8634, t8639)
}
