//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1799;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta446<F: Float>(t1307: F, t210: F, t6370: F, t1810: F, t5187: F, t6374: F, t1358: F, t6379: F, t19805: F, t554: F, t12211: F, t6371: F, t3726: F, t6375: F, t119: F, t19631: F, t12385: F, t6390: F, t16288: F, t1827: F, t1340: F, t19815: F, t12215: F, t1315: F, t1354: F, t16147: F, t16159: F, t16211: F, t16214: F, t16278: F, t16394: F, t3733: F, t5235: F, t5289: F, t5293: F, t5303: F, t559: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19823, t19827, t19831, t19834, t19836, t19839) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1799::<F>(t1307, t210, t6370, t1810, t5187, t6374, t1358, t6379, t19805, t554, t12211, t6371);
        let (t19841, t19844, t19851, t19853, t19855, t19862) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1800::<F>(t3726, t6375, t119, t19631, t210, t12385, t6390, t16288, t1827, t1340, t19815, t12215, t1315, t1354, t16147, t16159, t16211, t16214, t16278, t16394, t19823, t19827, t19831, t19834, t19836, t19839, t3733, t5235, t5289, t5293, t5303, t559);
    (t19823, t19827, t19831, t19834, t19836, t19839, t19841, t19844, t19851, t19853, t19855, t19862)
}
