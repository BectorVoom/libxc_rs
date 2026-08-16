//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2621;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta663<F: Float>(t21013: F, t3767: F, t3782: F, t3628: F, t4186: F, t5351: F, t3626: F, t12910: F, t17283: F, t17375: F, t17448: F, t17605: F, t1791: F, t21001: F, t21004: F, t21008: F, t3625: F, t5320: F, t5323: F, t5335: F, t5343: F, t5402: F, t5407: F) -> (F, F, F, F, F, F) {
        let (t21014, t21017) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2621::<F>(t21013, t3767, t3782);
        let (t21020, t21021, t21022, t21027) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2622::<F>(t3628, t4186, t5351, t3626, t12910, t17283, t17375, t17448, t17605, t1791, t21001, t21004, t21008, t21014, t21017, t3625, t5320, t5323, t5335, t5343, t5402, t5407);
    (t21014, t21017, t21020, t21021, t21022, t21027)
}
