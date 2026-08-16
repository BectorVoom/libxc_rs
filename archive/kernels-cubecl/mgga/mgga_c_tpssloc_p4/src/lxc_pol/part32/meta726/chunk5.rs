//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2346/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2346<F: Float>(t2110: F, t24517: F, t26009: F, t26016: F, t27298: F, t27937: F, t27979: F, t7256: F, t7259: F, t90114: F, t96102: F, t96110: F, t96115: F, t96120: F, t96383: F, t96443: F, t96646: F) -> F {
    let t104783 = -F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26016 * t96102 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26016 * t96110 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26016 * t96115 + t96646 * t2110 / F::cast_from(3.0_f64) + t27979 * t7256 / F::cast_from(3.0_f64) + t27979 * t7259 / F::cast_from(3.0_f64) - t96383 * t2110 / F::cast_from(6.0_f64) - t27937 * t7256 / F::cast_from(6.0_f64) - t27937 * t7259 / F::cast_from(6.0_f64) - F::cast_from(10.0_f64) * t96120 * t26009 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t90114 * t27298 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t96443 * t24517;
    t104783
}
