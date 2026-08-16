//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1373/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1373<F: Float>(t20288: F, t72: F, t79: F, t5398: F, t20218: F, t605: F, t1410: F, t19299: F, t1865: F, t26051: F, t27966: F, t27972: F, t27982: F, t6490: F, t7432: F, t7442: F, t7446: F, t96529: F, t96532: F, t96538: F, t96551: F) -> F {
    let t106849 = t72 * t79 * t20288;
    let t106853 = t72 * t79 * t5398;
    let t106855 = t605 * t20218;
    let t106862 = t19299 * t1410;
    let t106874 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6490 * t106849 + t96551 * t106853 + t106855 * t1865 / F::cast_from(3.0_f64) + t27982 * t7442 + t27982 * t7446 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t96532 * t7432 + t106862 * t1865 + F::cast_from(5.0_f64) * t96538 * t7432 + F::cast_from(2.0_f64) * t27966 * t7442 + F::cast_from(5.0_f64) * t26051 * t27972 + F::cast_from(2.0_f64) * t27966 * t7446 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t96529 * t7432;
    t106874
}
