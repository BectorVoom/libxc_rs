//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta176 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1057;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta176<F: Float>(t1799: F, t213: F, t1307: F, t221: F, t118: F, t794: F, t3739: F, t210: F, t214: F, t5187: F, t1315: F, t3725: F, t3727: F, t3731: F, t3742: F, t3751: F, t5192: F, t5195: F, t562: F, t1372: F, t1807: F, t1808: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t5196, t5198, t5202, t5203, t5206, t5210) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1057::<F>(t1799, t213, t1307, t221, t118, t794, t3739, t210, t214, t5187, t1315, t3725, t3727, t3731, t3742, t3751, t5192, t5195);
        let (t5211, t5213, t5215) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1058::<F>(t5210, t562, t1372, t1807, t1808, t225);
    (t5196, t5198, t5202, t5203, t5206, t5210, t5211, t5213, t5215)
}
