//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2266/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2266<F: Float>(t12571: F, t608: F, t33: F, t46099: F, t2244: F, t3953: F, t1865: F, t22513: F, t22516: F, t22534: F, t22551: F, t26016: F, t26028: F, t6506: F, t6510: F, t7428: F, t7442: F, t7446: F, t83725: F, t83729: F, t83738: F) -> F {
    let t90114 = t12571 * t608;
    let t90121 = t46099 * t33;
    let t90132 = t3953 * t2244;
    let t90135 = -F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26016 * t83725 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26016 * t83729 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26016 * t83738 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t90114 * t22551 + t22534 * t7446 / F::cast_from(3.0_f64) + t22534 * t7442 / F::cast_from(3.0_f64) - t90121 * t1865 / F::cast_from(6.0_f64) - t26028 * t6506 / F::cast_from(3.0_f64) - t26028 * t6510 / F::cast_from(3.0_f64) - t7428 * t22513 / F::cast_from(6.0_f64) - t7428 * t22516 / F::cast_from(3.0_f64) + t90132 * t1865 / F::cast_from(3.0_f64);
    t90135
}
