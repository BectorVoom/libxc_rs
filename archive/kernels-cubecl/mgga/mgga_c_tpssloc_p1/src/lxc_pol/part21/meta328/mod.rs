//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1701;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta328<F: Float>(t12283: F, t3809: F, t3777: F, t3789: F, t12248: F, t236: F, t3798: F, t1354: F, t12189: F, t1329: F, t3726: F, t3770: F, t12211: F, t3766: F, t1358: F, t3774: F, t1333: F, t3862: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12284, t12286, t12289, t12300) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1701::<F>(t12283, t3809, t3777, t3789, t12248, t236, t3798);
        let (t12301, t12308, t12310, t12317, t12323, t12325) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1702::<F>(t12300, t1354, t12189, t1329, t3726, t3770, t12211, t3766, t1358, t3774, t1333, t3862);
    (t12284, t12286, t12289, t12300, t12301, t12308, t12310, t12317, t12323, t12325)
}
