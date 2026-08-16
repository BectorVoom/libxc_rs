//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1370/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1370<F: Float>(t106758: F, t22544: F, t26013: F, t26016: F, t27937: F, t27950: F, t27953: F, t7428: F, t7442: F, t7446: F, t90137: F, t96426: F, t96443: F, t96454: F, t96462: F, t96470: F, t96473: F) -> F {
    let t106780 = -F::cast_from(15.0_f64) * t22544 * t106758 + F::cast_from(30.0_f64) * t90137 * t96426 - F::cast_from(10.0_f64) * t96443 * t26013 - F::cast_from(5.0_f64) * t96473 * t26013 - F::cast_from(10.0_f64) * t26016 * t96454 - F::cast_from(10.0_f64) * t26016 * t96462 - F::cast_from(5.0_f64) * t26016 * t96470 - t27937 * t7442 / F::cast_from(2.0_f64) - t27937 * t7446 / F::cast_from(2.0_f64) - t7428 * t27950 / F::cast_from(2.0_f64) - t7428 * t27953;
    t106780
}
