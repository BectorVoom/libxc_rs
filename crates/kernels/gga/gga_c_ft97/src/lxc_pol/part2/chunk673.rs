//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 673/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk673<F: Float>(t1882: F, t2528: F, t760: F, t255: F, t2576: F, t2571: F, t9895: F, t2492: F, t754: F, t2610: F, t8392: F, t9698: F) -> (F, F, F, F, F, F, F, F) {
    let t10048 = t1882 * t2528;
    let t10050 = t760 * t760;
    let t10051 = F::new(1.0) / t10050;
    let t10052 = t255 * t10051;
    let t10062 = t1882 * t2576;
    let t10064 = t1882 * t2571;
    let t10079 = t9895 * t255;
    let t10085 = t2492 * t754;
    let t10090 = t8392 * t2610;
    let t10119 = F::new(28.0) / F::new(27.0) * t9698;
    (t10048, t10052, t10062, t10064, t10079, t10085, t10090, t10119)
}
