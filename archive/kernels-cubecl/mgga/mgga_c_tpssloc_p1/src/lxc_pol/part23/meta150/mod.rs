//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk701;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk702;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta150<F: Float>(t1118: F, t6020: F, t1099: F, t3315: F, t5988: F, t3313: F, t3319: F, t4721: F, t5973: F, t5977: F, t5981: F, t1682: F, t1137: F, t3339: F, t3346: F, t4770: F, t5993: F, t6000: F, t6006: F, t6008: F, t6012: F, t6015: F, t6018: F, t3359: F, t3363: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6021, t6023, t6024, t6026, t6031, t6036) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk701::<F>(t1118, t6020, t1099, t3315, t5988, t3313, t3319, t4721, t5973, t5977, t5981, t1682);
        let (t6037, t6052) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk702::<F>(t1137, t6036, t3339, t3346, t4721, t4770, t5973, t5977, t5981, t5993, t6000, t6006, t6008, t6012, t6015, t6018);
        let (t6053, t6056, t6063) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk703::<F>(t1137, t6052, t3359, t6036, t3363, t4721, t5973, t5977, t5981);
    (t6021, t6023, t6024, t6026, t6031, t6036, t6037, t6052, t6053, t6056, t6063)
}
