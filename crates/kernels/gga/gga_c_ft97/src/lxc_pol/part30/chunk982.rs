//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 982/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk982<F: Float>(t34221: F, t681: F, t89: F, t1882: F, t34242: F, t34213: F, t7635: F, t8232: F, t7681: F, t34236: F, t2399: F, t7664: F) -> (F, F, F, F, F, F, F) {
    let t144190 = t89 * t681 * t34221;
    let t144197 = t1882 * t34242;
    let t144199 = t1882 * t34213;
    let t144212 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8232 * t7635;
    let t144219 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8232 * t7681;
    let t144227 = t1882 * t34236;
    let t144236 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t89 * t2399 * t7664;
    (t144190, t144197, t144199, t144212, t144219, t144227, t144236)
}
