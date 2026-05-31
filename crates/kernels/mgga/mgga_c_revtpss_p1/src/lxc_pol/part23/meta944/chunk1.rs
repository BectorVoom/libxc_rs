//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3099/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3099<F: Float>(t1149: F, t24324: F, t3384: F, t24323: F, t3435: F, t3433: F, t12227: F, t20651: F, t5104: F, t24220: F, t44091: F, t44093: F) -> (F, F, F, F) {
    let t81649 = F::cast_from(2.0_f64) * t3384 * t24324 * t1149;
    let t81650 = t24323 * t3435;
    let t81653 = F::cast_from(0.16081979498692535067e2_f64) * t3433 * t81650 * t1149;
    let t81656 = F::cast_from(0.1551780387578202009e4_f64) * t12227 * t20651 * t5104;
    let t81660 = F::cast_from(0.24955700379505800916e5_f64) * t44091 * t24220 * t44093 * t1149;
    (t81649, t81653, t81656, t81660)
}
