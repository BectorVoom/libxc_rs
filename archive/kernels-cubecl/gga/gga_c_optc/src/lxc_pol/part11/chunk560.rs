//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 560/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk560<F: Float>(t50: F, t1896: F, t4570: F, t4573: F, t52: F, t4569: F, t59: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t4577 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1896 * t4570 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t4573);
    let t4579 = (t4569 + t4577) * t59;
    t4579
}
