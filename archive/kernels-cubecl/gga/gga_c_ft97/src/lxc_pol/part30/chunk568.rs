//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 568/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk568<F: Float>(t1503: F, t8232: F, t1882: F, t6355: F, t6280: F, t6289: F, t1497: F, t2399: F, t89: F, t6347: F, t870: F) -> (F, F, F, F, F, F) {
    let t25194 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8232 * t1503;
    let t25195 = t1882 * t6355;
    let t25246 = t1882 * t6280;
    let t25248 = t1882 * t6289;
    let t25252 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t89 * t2399 * t1497;
    let t25253 = t6347 * t870;
    (t25194, t25195, t25246, t25248, t25252, t25253)
}
