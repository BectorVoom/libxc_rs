//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1437;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1438;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta286<F: Float>(t13176: F, t816: F, t1512: F, t9671: F, t2697: F, t4257: F, t2563: F, t4159: F, t4155: F, t9573: F, t2644: F, t820: F, t1509: F, t828: F, t2632: F, t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F, t2642: F, t4166: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13177, t13182, t13190, t13202, t13208, t13222) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1437::<F>(t13176, t816, t1512, t9671, t2697, t4257, t2563, t4159, t4155, t9573, t2644, t820);
        let (t13223, t13228) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1438::<F>(t1509, t828, t2632);
        let (t13234, t13237, t13242, t13251) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1439::<F>(t1500, t2693, t4163, t838, t120, t4233, t2642, t4166);
    (t13177, t13182, t13190, t13202, t13208, t13222, t13223, t13228, t13234, t13237, t13242, t13251)
}
