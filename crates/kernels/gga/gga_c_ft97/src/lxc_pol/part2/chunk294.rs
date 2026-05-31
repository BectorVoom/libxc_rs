//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 294/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk294<F: Float>(t1240: F, t295: F, t312: F, t1188: F, t1215: F, t1236: F, t873: F) -> (F, F) {
    let t1242 = t295 * t1240 * t312;
    let t1248 = t1236 / F::cast_from(2.0_f64) - t873 - t1188 / F::cast_from(3.0_f64) - t1215;
    (t1242, t1248)
}
