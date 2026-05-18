//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 645/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk645<F: Float>(t2247: F, t47: F, t68: F, t72: F, t424: F, t626: F, t419: F, t1570: F, t23: F, t10: F, t3050: F, t83: F) -> (F, F, F, F, F) {
    let t8076 = t47 * t2247;
    let t8078 = t68 * t8076 * t72;
    let t8079 = F::new(0.70937342644032921812e-2) * t8078;
    let t8109 = t626 * t424;
    let t8110 = t419 * t8109;
    let t8119 = F::new(1.0) / t23 / t1570;
    let t8189 = t10 * t3050 * t83;
    (t8078, t8079, t8110, t8119, t8189)
}
