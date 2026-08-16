//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2026;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2027;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta461<F: Float>(t225: F, t5213: F, t1807: F, t3879: F, t5211: F, t1332: F, t5343: F, t1372: F, t1824: F, t5250: F, t5286: F, t562: F, t3851: F, t5335: F, t12248: F, t68: F, t544: F, t12250: F, t3791: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16022, t16028, t16030, t16033) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2026::<F>(t225, t5213, t1807, t3879, t5211, t1332, t5343);
        let (t16036, t16037, t16040, t16041, t16044, t16046, t16047) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2027::<F>(t1372, t1824, t5250, t5286, t562, t3851, t5335, t12248, t68, t544);
        let t16048 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2028::<F>(t12250, t3791);
    (t16022, t16028, t16030, t16033, t16036, t16037, t16040, t16041, t16044, t16046, t16047, t16048)
}
