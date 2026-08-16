//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1249/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1249<F: Float>(t1044: F, t1149: F, t12365: F, t12964: F, t44147: F, t44150: F, t44152: F, t44155: F, t44158: F, t44161: F, t44165: F, t44168: F, t44519: F, t44524: F, t44526: F, t44530: F, t44532: F, t44535: F, t860: F, t9782: F) -> F {
    let t44536 = F::cast_from(2.0_f64) * t1044 * t12365 + t1149 * t9782 + t12964 * t860 + t44147 - t44150 + t44152 + t44155 + t44158 - t44161 + t44165 + t44168 + t44519 - t44524 - t44526 + t44530 - t44532 - t44535;
    t44536
}
