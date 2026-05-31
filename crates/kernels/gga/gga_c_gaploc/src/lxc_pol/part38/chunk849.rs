//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 849/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk849<F: Float>(t39624: F, t39626: F, t39632: F, t39637: F, t39642: F, t39646: F, t39648: F, t39650: F, t471: F, t13287: F, t64: F, t11210: F, t871: F) -> (F, F, F) {
    let t44590 = (F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t39624 + F::cast_from(357.0_f64) / F::cast_from(8192.0_f64) * t39626 - F::cast_from(189.0_f64) / F::cast_from(131072.0_f64) * t39632 + F::cast_from(189.0_f64) / F::cast_from(8388608.0_f64) * t39637 - F::cast_from(63.0_f64) / F::cast_from(8388608.0_f64) * t39642 + F::cast_from(63.0_f64) / F::cast_from(131072.0_f64) * t39646 - F::cast_from(119.0_f64) / F::cast_from(8192.0_f64) * t39648 - F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t39650) * t471;
    let t44592 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13287 * t64;
    let t44593 = t11210 * t871;
    (t44590, t44592, t44593)
}
