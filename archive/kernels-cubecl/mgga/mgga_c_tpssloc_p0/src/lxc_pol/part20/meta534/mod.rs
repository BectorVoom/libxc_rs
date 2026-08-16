//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta534<F: Float>(t12204: F, t40409: F, t12214: F, t792: F, t118: F, t12156: F, t794: F, t2229: F, t59: F, t60: F, t535: F, t9538: F, t12231: F, t3726: F, t12199: F, t12208: F, t12012: F, t3739: F, t12217: F, t40021: F, t3774: F, t3862: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40410, t40415, t40419, t40422) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2071::<F>(t12204, t40409, t12214, t792, t118, t12156, t794, t2229, t59, t60, t535, t9538);
        let (t40423, t40425, t40429, t40431, t40443) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2072::<F>(t12231, t3726, t12199, t12208, t118, t12012, t3739, t794, t12217, t40021, t3774, t3862);
    (t40410, t40415, t40419, t40422, t40423, t40425, t40429, t40431, t40443)
}
