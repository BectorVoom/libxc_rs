//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2535;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2536;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta704<F: Float>(t1036: F, t13751: F, t10422: F, t14229: F, t3070: F, t14234: F, t42488: F, t1022: F, t4649: F, t41666: F, t43398: F, t14036: F, t13969: F, t13976: F, t3130: F, t1041: F, t14183: F, t10471: F, t47840: F, t10479: F, t10908: F, t4641: F, t10216: F, t13797: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48446, t48460, t48463, t48477, t48496, t48548) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2535::<F>(t1036, t13751, t10422, t14229, t3070, t14234, t42488, t1022, t4649, t41666, t43398, t14036);
        let (t48564, t48567, t48569, t48570, t48574, t48585) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2536::<F>(t13969, t13976, t3130, t1041, t14183, t10471, t47840, t10479, t10908, t4641, t10216, t13797);
    (t48446, t48460, t48463, t48477, t48496, t48548, t48564, t48567, t48569, t48570, t48574, t48585)
}
