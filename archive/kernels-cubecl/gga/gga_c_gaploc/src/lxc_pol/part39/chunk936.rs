//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 936/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk936<F: Float>(t12925: F, t4614: F, t574: F, t3133: F, t4752: F, t8352: F, t12887: F, t1641: F, t39624: F, t39626: F, t39632: F, t39637: F, t39642: F, t39646: F, t39648: F, t39650: F, t471: F) -> (F, F, F, F) {
    let t42081 = F::cast_from(0.61348681526273199483e1_f64) * t574 * t4614 * t12925;
    let t42092 = F::cast_from(0.28600391961480341335e1_f64) * t8352 * t4752 * t3133;
    let t42099 = F::cast_from(0.92023022289409799224e1_f64) * t1641 * t12887;
    let t42111 = (F::cast_from(21.0_f64) / F::cast_from(512.0_f64) * t39624 + F::cast_from(357.0_f64) / F::cast_from(16384.0_f64) * t39626 - F::cast_from(189.0_f64) / F::cast_from(262144.0_f64) * t39632 + F::cast_from(189.0_f64) / F::cast_from(0.16777216e8_f64) * t39637 - F::cast_from(63.0_f64) / F::cast_from(0.16777216e8_f64) * t39642 + F::cast_from(63.0_f64) / F::cast_from(262144.0_f64) * t39646 - F::cast_from(119.0_f64) / F::cast_from(16384.0_f64) * t39648 - F::cast_from(7.0_f64) / F::cast_from(512.0_f64) * t39650) * t471;
    (t42081, t42092, t42099, t42111)
}
