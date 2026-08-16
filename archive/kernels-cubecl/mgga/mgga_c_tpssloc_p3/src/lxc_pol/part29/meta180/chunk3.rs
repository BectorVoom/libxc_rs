//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 954/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk954<F: Float>(t3990: F, t607: F, t3966: F, t55: F, t1414: F, t1420: F, t2282: F, t39: F, t3982: F, t3985: F, t51: F, t615: F, t621: F) -> F {
    let t3991 = t3990 * t607;
    let t3994 = t55 * t3966;
    let t3997 = -F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t615 * t1414 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t39 * t3982 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t3985 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t1420 * t621 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t51 * t3991 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t3994 - t2282;
    t3997
}
