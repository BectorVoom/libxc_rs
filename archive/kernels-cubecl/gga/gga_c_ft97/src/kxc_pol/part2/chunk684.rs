//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 684/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk684<F: Float>(t10282: F, t10286: F, t10243: F, t10397: F, t2832: F, t870: F, t1882: F, t2859: F, t2854: F, t192: F, t7640: F, t2842: F, t863: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10641 = t10282 / F::cast_from(9.0_f64);
    let t10643 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10286;
    let t10649 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10243;
    let t10658 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t10397;
    let t10666 = t2832 * t870;
    let t10670 = t1882 * t2859;
    let t10678 = t1882 * t2854;
    let t10683 = t192 * t7640;
    let t10688 = t863 * t2842;
    (t10641, t10643, t10649, t10658, t10666, t10670, t10678, t10683, t10688)
}
