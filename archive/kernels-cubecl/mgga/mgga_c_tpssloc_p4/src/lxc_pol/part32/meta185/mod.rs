//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta185 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk901;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk902;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk903;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta185<F: Float>(t1216: F, t1653: F, t3578: F, t1222: F, t1731: F, t1744: F, t1202: F, t1743: F, t225: F, t4940: F, t68: F, t484: F, t1177: F, t4729: F, t1229: F, t3247: F, t3961: F, t4582: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4953, t4954) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk901::<F>(t1216, t1653, t3578);
        let (t4957, t4959, t4961, t4964) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk902::<F>(t1222, t1731, t1744, t1202, t1743, t225, t4940);
        let t4965 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk903::<F>(t4964, t68);
        let (t4966, t4969, t4972, t4973, t4974) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk904::<F>(t484, t4965, t1177, t4729, t1229, t3247, t3961, t4582);
    (t4953, t4954, t4957, t4959, t4961, t4964, t4965, t4966, t4969, t4972, t4973, t4974)
}
