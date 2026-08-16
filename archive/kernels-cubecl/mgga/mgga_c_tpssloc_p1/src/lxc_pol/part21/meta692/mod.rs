//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2507;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta692<F: Float>(t13176: F, t2696: F, t849: F, t13360: F, t2707: F, t1509: F, t9975: F, t242: F, t41347: F, t812: F, t13297: F, t9573: F, t13080: F, t9638: F, t226: F, t40931: F, t68: F, t13377: F, t814: F, t13396: F, t808: F, t13068: F, t225: F, t13030: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47278, t47279, t47283, t47285, t47307, t47333) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2507::<F>(t13176, t2696, t849, t13360, t2707, t1509, t9975, t242, t41347, t812, t13297, t9573);
        let (t47353, t47386, t47395, t47419, t47568, t47585) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2508::<F>(t13080, t9638, t226, t40931, t68, t13377, t814, t13396, t808, t13068, t225, t13030);
    (t47278, t47279, t47283, t47285, t47307, t47333, t47353, t47386, t47395, t47419, t47568, t47585)
}
