//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2341/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2341<F: Float>(t2240: F, t27363: F, t33: F, t24520: F, t24526: F, t26063: F, t26067: F, t27308: F, t27311: F, t27365: F, t6492: F, t6495: F, t7246: F, t90177: F, t90227: F, t90232: F, t90334: F) -> F {
    let t96072 = t2240 * t33 * t27363;
    let t96077 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t27308 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7246 * t90177 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t27311 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24520 * t26063 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24526 * t26063 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t90227 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7246 * t90232 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24520 * t26067 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24526 * t26067 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t90334 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t96072 * t6492 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t27365;
    t96077
}
