//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 570/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk570<F: Float>(t5299: F, t799: F, t27: F, t89: F, t2653: F, t4230: F, t4235: F, t5211: F, t5215: F, t5219: F, t5223: F, t5228: F, t788: F, t1248: F) -> (F, F, F, F, F) {
    let t5300 = t799 * t5299;
    let t5302 = t89 * t27 * t5300;
    let t5304 = t2653 + t4230 + t4235 - t5211 / 27.0 + t5215 / 9.0 + t5219 / 9.0 - t5223 / 18.0 + t5228 / 3.0 - t5302 / 6.0;
    let t5305 = t788 * t5304;
    let t5309 = t1248 * t1248;
    (t5300, t5302, t5304, t5305, t5309)
}
