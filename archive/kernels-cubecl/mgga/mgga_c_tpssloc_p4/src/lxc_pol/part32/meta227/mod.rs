//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta227 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta227<F: Float>(t3297: F, t5971: F, t136: F, t1113: F, t5975: F, t5979: F, t3282: F, t3294: F, t4721: F, t4770: F, t5973: F, t5977: F, t5981: F, t5993: F, t6000: F, t6006: F, t6008: F) -> (F, F, F, F, F, F, F) {
        let (t6011, t6012, t6014, t6015, t6017, t6018, t6020) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1045::<F>(t3297, t5971, t136, t1113, t5975, t5979, t3282, t3294, t4721, t4770, t5973, t5977, t5981, t5993, t6000, t6006, t6008);
    (t6011, t6012, t6014, t6015, t6017, t6018, t6020)
}
