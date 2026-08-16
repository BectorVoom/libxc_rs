//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2185;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta604<F: Float>(t11719: F, t11722: F, t248: F, t3570: F, t11818: F, t1213: F, t3494: F, t3506: F, t3509: F, t3515: F, t3516: F, t11718: F, t44857: F, t11661: F, t13969: F, t11721: F, t3493: F, t11858: F, t1226: F, t3030: F, t3481: F, t3032: F, t3505: F, t3514: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44871, t44886, t44890, t44894, t44896) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2185::<F>(t11719, t11722, t248, t3570, t11818, t1213, t3494, t3506, t3509, t3515, t3516, t11718, t44857);
        let (t44904, t44906, t44918, t44927, t44929, t44932) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2186::<F>(t11661, t13969, t3506, t11721, t3493, t11858, t1226, t3030, t3481, t3032, t3505, t3514);
    (t44871, t44886, t44890, t44894, t44896, t44904, t44906, t44918, t44927, t44929, t44932)
}
