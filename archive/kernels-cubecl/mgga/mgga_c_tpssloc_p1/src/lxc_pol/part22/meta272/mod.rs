//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta272<F: Float>(t3777: F, t3789: F, t12248: F, t236: F, t240: F, t1336: F, t3798: F, t12189: F, t1329: F, t1333: F, t3862: F, t10022: F, t248: F, t557: F) -> (F, F, F, F, F, F, F, F) {
        let (t12286, t12289, t12290, t12291, t12300, t12308, t12325, t12328) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1420::<F>(t3777, t3789, t12248, t236, t240, t1336, t3798, t12189, t1329, t1333, t3862, t10022, t248, t557);
    (t12286, t12289, t12290, t12291, t12300, t12308, t12325, t12328)
}
