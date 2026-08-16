//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2004;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta610<F: Float>(t10336: F, t1920: F, t1922: F, t1049: F, t23592: F, t10164: F, t225: F, t1921: F, t6733: F, t3034: F, t336: F, t131: F, t350: F, t38: F, t10469: F, t344: F, t10482: F, t3032: F, t23598: F, t614: F, t3131: F, t23383: F, t6712: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t82436, t82469, t82481, t82502, t82513) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2004::<F>(t10336, t1920, t1922, t1049, t23592, t10164, t225, t1921, t6733, t3034, t336, t131, t350, t38);
        let (t82514, t82516, t82534, t82542, t82573) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2005::<F>(t10469, t344, t10482, t3032, t131, t23598, t350, t614, t3131, t23383, t6712);
    (t82436, t82469, t82481, t82502, t82513, t82514, t82516, t82534, t82542, t82573)
}
