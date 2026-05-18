//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1060/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1060<F: Float>(t36766: F, t8443: F, t4601: F, t8884: F, t2191: F, t8582: F, t2868: F, t7855: F, t2057: F, t26370: F, t9000: F, t9128: F) -> (F, F, F, F, F, F) {
    let t41964 = t36766 * t8443;
    let t41969 = t4601 * t8884;
    let t41971 = t2191 * t8582;
    let t41973 = t2868 * t7855;
    let t41975 = t26370 * t2057;
    let t41977 = t9128 * t9000;
    (t41964, t41969, t41971, t41973, t41975, t41977)
}
