//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 990/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk990<F: Float>(t66202: F, t80096: F, t88737: F, t88740: F, t88744: F, t88747: F, t88751: F, t88754: F, t88758: F, t88761: F, t88766: F, t88769: F, t88734: F, t223: F, t80002: F, t52358: F) -> (F, F, F) {
    let t88772 = -8.0 * t88737 + 8.0 * t88740 - 2.0 / 3.0 * t88744 - 8.0 / 9.0 * t88747 + 8.0 * t88751 - 12.0 * t88754 + 2.0 * t88758 + 8.0 / 3.0 * t88761 - 8.0 / 9.0 * t66202 + 40.0 / 9.0 * t88766 - 20.0 / 9.0 * t88769 + 4.0 / 9.0 * t80096;
    let t88773 = t88734 + t88772;
    let t88796 = t80002 * t223;
    let t88797 = t52358 * t88796;
    (t88773, t88796, t88797)
}
