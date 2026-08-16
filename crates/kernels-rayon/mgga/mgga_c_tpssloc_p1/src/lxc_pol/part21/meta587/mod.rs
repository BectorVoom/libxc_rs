//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2323;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2324;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2325;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta587(t1307: f64, t210: f64, t6370: f64, t1810: f64, t5187: f64, t6374: f64, t1358: f64, t6379: f64, t19805: f64, t554: f64, t12211: f64, t6371: f64, t3726: f64, t6375: f64, t119: f64, t19631: f64, t12385: f64, t6390: f64, t16288: f64, t1827: f64, t1340: f64, t19815: f64, t12215: f64, t1315: f64, t1354: f64, t16147: f64, t16159: f64, t16211: f64, t16214: f64, t16278: f64, t16394: f64, t3733: f64, t5235: f64, t5289: f64, t5293: f64, t5303: f64, t559: f64, t1343: f64, t19732: f64, t820: f64, t120: f64, t6387: f64, t5248: f64, t5250: f64, t5234: f64, t5245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19823, t19827, t19831, t19834, t19836, t19839) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2323(t1307, t210, t6370, t1810, t5187, t6374, t1358, t6379, t19805, t554, t12211, t6371);
        let (t19843, t19844, t19855, t19862) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2324(t3726, t6375, t119, t19631, t210, t12385, t6390, t16288, t1827, t1340, t19815, t12215, t1315, t1354, t16147, t16159, t16211, t16214, t16278, t16394, t19823, t19827, t19831, t19834, t19836, t19839, t3733, t5235, t5289, t5293, t5303, t559);
        let (t19868, t19871) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2325(t1343, t19732, t820, t120, t6387);
        let (t19873, t19876) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2326(t19871, t5248, t5250, t5234, t5245);
    (t19823, t19827, t19831, t19836, t19843, t19844, t19855, t19862, t19868, t19871, t19873, t19876)
}
