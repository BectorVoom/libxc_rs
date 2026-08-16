//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2323;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2324;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2325;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta587<F: Float>(t1307: F, t210: F, t6370: F, t1810: F, t5187: F, t6374: F, t1358: F, t6379: F, t19805: F, t554: F, t12211: F, t6371: F, t3726: F, t6375: F, t119: F, t19631: F, t12385: F, t6390: F, t16288: F, t1827: F, t1340: F, t19815: F, t12215: F, t1315: F, t1354: F, t16147: F, t16159: F, t16211: F, t16214: F, t16278: F, t16394: F, t3733: F, t5235: F, t5289: F, t5293: F, t5303: F, t559: F, t1343: F, t19732: F, t820: F, t120: F, t6387: F, t5248: F, t5250: F, t5234: F, t5245: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19823, t19827, t19831, t19834, t19836, t19839) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2323::<F>(t1307, t210, t6370, t1810, t5187, t6374, t1358, t6379, t19805, t554, t12211, t6371);
        let (t19843, t19844, t19855, t19862) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2324::<F>(t3726, t6375, t119, t19631, t210, t12385, t6390, t16288, t1827, t1340, t19815, t12215, t1315, t1354, t16147, t16159, t16211, t16214, t16278, t16394, t19823, t19827, t19831, t19834, t19836, t19839, t3733, t5235, t5289, t5293, t5303, t559);
        let (t19868, t19871) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2325::<F>(t1343, t19732, t820, t120, t6387);
        let (t19873, t19876) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2326::<F>(t19871, t5248, t5250, t5234, t5245);
    (t19823, t19827, t19831, t19836, t19843, t19844, t19855, t19862, t19868, t19871, t19873, t19876)
}
