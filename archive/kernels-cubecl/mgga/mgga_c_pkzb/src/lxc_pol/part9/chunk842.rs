//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 842/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk842<F: Float>(t24: F, t2177: F, t91: F, t2179: F, t507: F, t1655: F, t5107: F, t5113: F, t821: F, zeta_threshold: F) -> (F, F) {
    let t90 = t24 <= zeta_threshold;
    let t6097 = F::cast_from(1.0_f64) / t91 / t2177 / t24;
    let t6100 = t2179 * t507;
    let t6106 = piecewise3::<F>(t90, F::cast_from(0.0_f64), -F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t6097 * t5107 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t6100 * t1655 - t821 * t5113 / F::cast_from(3.0_f64));
    (t6097, t6106)
}
