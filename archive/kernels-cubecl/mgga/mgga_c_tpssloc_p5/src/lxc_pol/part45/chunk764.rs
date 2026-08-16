//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 764/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk764<F: Float>(t23232: F, t22975: F, t22979: F, t23191: F, t23198: F, t23202: F, t23207: F, t23209: F, t23211: F, t23215: F, t23220: F, t23224: F, t23226: F, t23231: F, t259: F, t2597: F, t2713: F, t6632: F, t6663: F, t855: F) -> F {
    let t23233 = F::cast_from(0.76763589786250567036e-1_f64) * t23232;
    let t23234 = F::cast_from(4.0_f64) * t2713 * t6632 + F::cast_from(2.0_f64) * t855 * t22975 + F::cast_from(4.0_f64) * t855 * t22979 - t855 * t23191 - F::cast_from(2.0_f64) * t2713 * t6663 + F::cast_from(0.16449340668482264365e-1_f64) * t23198 + F::cast_from(4.0_f64) * t2597 * t6632 + t23202 * t259 + t23207 + F::cast_from(0.82246703342411321824e-2_f64) * t23209 + F::cast_from(2.0_f64) * t23211 * t259 - F::cast_from(6.0_f64) * t855 * t23215 - F::cast_from(0.82246703342411321825e-2_f64) * t23220 - F::cast_from(0.16449340668482264365e-1_f64) * t23224 + t23226 * t259 - t23231 + t23233;
    t23234
}
