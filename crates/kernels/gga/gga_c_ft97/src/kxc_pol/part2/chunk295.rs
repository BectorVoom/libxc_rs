//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 295/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk295<F: Float>(t1248: F, t871: F, t296: F, t1221: F, t1225: F, t1242: F, t193: F, t446: F, t834: F, t89: F, t1240: F, t312: F) -> (F, F, F, F) {
    let t1249 = t871 * t1248;
    let t1250 = t296 * t1249;
    let t1253 = -t834 - t446 * t1221 / F::new(9.0) - t446 * t1225 / F::new(3.0) + t89 * t193 * t1242 / F::new(3.0) - t446 * t1250 / F::new(3.0);
    let t1255 = t1240 * t312;
    (t1249, t1250, t1253, t1255)
}
