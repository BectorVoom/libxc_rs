//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 913/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk913<F: Float>(t3871: F, t8392: F, t255: F, t676: F, t1168: F, t2567: F, t2579: F, t1131: F, t2373: F, t10157: F, t265: F, t12001: F, t3852: F) -> (F, F, F, F) {
    let t14126 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t3871;
    let t14127 = t676 * t255;
    let t14128 = t2567 * t1168;
    let t14129 = t14128 * t2579;
    let t14130 = t14127 * t14129;
    let t14133 = t1131 * t2373;
    let t14135 = t10157 * t265 * t14133;
    let t14138 = t12001 * t3852;
    (t14126, t14130, t14135, t14138)
}
