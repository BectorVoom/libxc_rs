//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 435/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk435<F: Float>(t2413: F, t683: F, t92: F, t2401: F, t2402: F, t2407: F, t2411: F) -> (F, F, F) {
    let t2414 = t683 * t2413;
    let t2415 = t92 * t2414;
    let t2417 = t2401 + F::new(2.0) / F::new(9.0) * t2402 - F::new(2.0) / F::new(9.0) * t2407 + F::new(2.0) / F::new(3.0) * t2411 - t2415 / F::new(3.0);
    (t2414, t2415, t2417)
}
