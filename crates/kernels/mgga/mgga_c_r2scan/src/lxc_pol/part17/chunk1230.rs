//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1230/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1230<F: Float>(t39857: F, t43281: F, t43284: F, t43286: F, t43288: F, t43291: F, t43294: F, t43296: F, t43299: F, t43302: F, t43305: F, t43308: F) -> F {
    let t44330 = -F::cast_from(0.55476619136347318804e1_f64) * t39857 + F::cast_from(0.95219938395347901947e-2_f64) * t43281 + F::cast_from(0.47609969197673950973e-2_f64) * t43284 + F::cast_from(0.28565981518604370584e-1_f64) * t43286 - F::cast_from(0.32927245914677557992e-1_f64) * t43288 - F::cast_from(0.26198215989259945076e-1_f64) * t43291 + F::cast_from(0.34672886960217074252e0_f64) * t43294 + F::cast_from(0.5200933044032561138e0_f64) * t43296 + F::cast_from(0.5200933044032561138e0_f64) * t43299 + F::cast_from(0.5200933044032561138e0_f64) * t43302 - F::cast_from(0.10401866088065122276e1_f64) * t43305 - F::cast_from(0.5200933044032561138e1_f64) * t43308;
    t44330
}
