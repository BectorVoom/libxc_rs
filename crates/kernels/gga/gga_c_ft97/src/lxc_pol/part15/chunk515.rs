//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 515/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk515<F: Float>(t1212: F, t231: F, t1218: F, t1526: F, t2320: F, t2638: F, t342: F, t343: F, t5198: F, t1217: F, t2660: F, t4917: F) -> (F, F, F, F) {
    let t5202 = t231 * t1212;
    let t5206 = t1218 - t2638 - t1526 * t2320 * t5198 / F::cast_from(12.0_f64) - t342 * t343 * t5202 / F::cast_from(4.0_f64);
    let t5207 = t5206 * t1217;
    let t5209 = t2660 * t4917;
    (t5202, t5206, t5207, t5209)
}
