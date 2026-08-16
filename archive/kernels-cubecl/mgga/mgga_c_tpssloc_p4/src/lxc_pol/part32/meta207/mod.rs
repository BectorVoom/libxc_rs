//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk994;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk995;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk996;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta207<F: Float>(t252: F, t5558: F, t1492: F, t1519: F, t119: F, t5527: F, t210: F, t5544: F, t225: F, t237: F, t1509: F, t2632: F, t819: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5559, t5561, t5567, t5568, t5571, t5572, t5575) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk994::<F>(t252, t5558, t1492, t1519, t119, t5527, t210, t5544, t225);
        let (t5576, t5584) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk995::<F>(t237, t5575, t1509);
        let t5585 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk996::<F>(t2632, t5584);
        let t5587 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk997::<F>(t5585, t819, t820);
    (t5559, t5561, t5567, t5568, t5571, t5572, t5575, t5576, t5584, t5585, t5587)
}
