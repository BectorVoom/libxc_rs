//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 574/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk574<F: Float>(t109: F, t5488: F, t656: F, t2327: F, t4041: F, t5465: F, t64: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t5489 = t656 * t5488;
    let t5493 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t2327 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4041 + t64 * t5465 / F::cast_from(4.0_f64) - t64 * t5489 / F::cast_from(8.0_f64));
    (t5489, t5493)
}
