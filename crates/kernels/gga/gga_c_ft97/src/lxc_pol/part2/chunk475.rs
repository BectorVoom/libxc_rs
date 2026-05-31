//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 475/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk475<F: Float>(t2739: F, t799: F, t27: F, t89: F, t2653: F, t2656: F, t2659: F, t2663: F, t2668: F, t2673: F, t2677: F, t2685: F) -> (F, F, F) {
    let t2740 = t799 * t2739;
    let t2742 = t89 * t27 * t2740;
    let t2744 = t2653 + t2656 + t2659 - t2663 / F::cast_from(27.0_f64) + t2668 / F::cast_from(9.0_f64) + t2673 / F::cast_from(9.0_f64) - t2677 / F::cast_from(18.0_f64) + t2685 / F::cast_from(3.0_f64) - t2742 / F::cast_from(6.0_f64);
    (t2740, t2742, t2744)
}
