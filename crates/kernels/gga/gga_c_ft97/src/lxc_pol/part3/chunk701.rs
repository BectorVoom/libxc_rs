//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 701/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk701<F: Float>(t1008: F, t2057: F, t550: F, t1995: F, t527: F, t11260: F, t1018: F, t1636: F, t89: F, t1026: F, t8232: F, t1882: F, t3463: F) -> (F, F, F, F, F, F, F) {
    let t12401 = t2057 * t1008;
    let t12448 = t550 * t1008;
    let t12449 = t1995 * t12448;
    let t12452 = t527 * t12448;
    let t12527 = F::cast_from(0.22226000364197530866e-1_f64) * t11260;
    let t12571 = t89 * t1636 * t1018;
    let t12617 = t8232 * t1026;
    let t12620 = F::new(2.0) / F::new(27.0) * t1882 * t3463;
    (t12401, t12449, t12452, t12527, t12571, t12617, t12620)
}
