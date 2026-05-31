//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 983/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk983<F: Float>(t12: F, t10760: F, t10764: F, t2732: F, t3366: F, zeta_threshold: F) -> F {
    let t84 = t12 <= zeta_threshold;
    let t10767 = piecewise3::<F>(t84, F::cast_from(0.0_f64), -F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t10760 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2732 * t3366 - t10764 / F::cast_from(3.0_f64));
    t10767
}
