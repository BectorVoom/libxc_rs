//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 753/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk753<F: Float>(t12925: F, t4614: F, t574: F, t12792: F, t203: F, t447: F, t3133: F, t4752: F, t8352: F, t1564: F, t12887: F, t1641: F, t39624: F, t39626: F, t39632: F, t39637: F, t39642: F, t39646: F, t39648: F, t39650: F, t471: F) -> (F, F, F, F, F, F, F) {
    let t42081 = 0.61348681526273199483e1 * t574 * t4614 * t12925;
    let t42085 = t203 * t12792;
    let t42086 = t42085 * t447;
    let t42092 = 0.28600391961480341335e1 * t8352 * t4752 * t3133;
    let t42093 = t1564 * t12792;
    let t42099 = 0.92023022289409799224e1 * t1641 * t12887;
    let t42111 = (21.0 / 512.0 * t39624 + 357.0 / 16384.0 * t39626 - 189.0 / 262144.0 * t39632 + 189.0 / 0.16777216e8 * t39637 - 63.0 / 0.16777216e8 * t39642 + 63.0 / 262144.0 * t39646 - 119.0 / 16384.0 * t39648 - 7.0 / 512.0 * t39650) * t471;
    (t42081, t42085, t42086, t42092, t42093, t42099, t42111)
}
