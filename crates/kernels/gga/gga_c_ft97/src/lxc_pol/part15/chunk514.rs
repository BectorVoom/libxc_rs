//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 514/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk514<F: Float>(t1137: F, t1173: F, t247: F, t263: F, t4915: F, t5059: F, t5065: F, t5148: F, t5152: F, t5179: F, t5181: F, t2639: F, t992: F) -> (F, F) {
    let t5186 = -F::new(2.0) * t1137 * t1173 - t247 * t5179 - t263 * t4915 - t263 * t5059 + F::new(4.0) * t5065 - F::new(2.0) * t5148 - F::new(4.0) * t5152 + F::new(2.0) * t5181;
    let t5198 = t2639 * t992;
    (t5186, t5198)
}
