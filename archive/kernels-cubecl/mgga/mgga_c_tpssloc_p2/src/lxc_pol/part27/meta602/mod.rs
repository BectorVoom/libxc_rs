//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta602<F: Float>(t1920: F, t23353: F, t968: F, t10164: F, t225: F, t23384: F, t23595: F, t23408: F, t1921: F, t6733: F, t3034: F, t336: F, t131: F, t350: F, t38: F, t10469: F, t344: F, t10482: F, t3032: F, t2261: F, t6794: F, t23598: F, t614: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t82463, t82481, t82490, t82499, t82502, t82510) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2071::<F>(t1920, t23353, t968, t10164, t225, t23384, t23595, t23408, t1921, t6733, t3034, t336);
        let (t82513, t82514, t82516, t82527, t82534) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2072::<F>(t131, t350, t38, t82510, t10469, t344, t10482, t3032, t2261, t6794, t23598, t614);
    (t82463, t82481, t82490, t82499, t82502, t82513, t82514, t82516, t82527, t82534)
}
