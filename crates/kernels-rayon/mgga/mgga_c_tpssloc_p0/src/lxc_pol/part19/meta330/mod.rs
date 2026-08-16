//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1177;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1178;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1179;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta330(t12222: f64, t16081: f64, t116: f64, t1314: f64, t9534: f64, t1307: f64, t133: f64, t6600: f64, t12226: f64, t16094: f64, t3719: f64, t686: f64, t3736: f64, t40018: f64, t12012: f64, t12220: f64, t16101: f64, t210: f64, t213: f64, t214: f64, t221: f64, t3733: f64, t3734: f64, t39622: f64, t40343: f64, t40347: f64, t40350: f64, t40351: f64, t40356: f64, t40360: f64, t5195: f64, t59: f64, t9223: f64, t120: f64, t212: f64, t22815: f64, t67: f64, t535: f64, t1317: f64, t40005: f64, t12189: f64, t3745: f64, t9580: f64, t3741: f64, t2566: f64, t3732: f64, t12204: f64, t12214: f64, t792: f64, t118: f64, t12156: f64, t794: f64, t2229: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40366, t40372, t40376) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1177(t12222, t16081, t116, t1314, t9534, t1307, t133, t6600, t12226, t16094, t3719, t686);
        let t40389 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1178(t3736, t40018, t12012, t12220, t16101, t210, t213, t214, t221, t3719, t3733, t3734, t39622, t40343, t40347, t40350, t40351, t40356, t40360, t40366, t40372, t40376, t5195);
        let (t40394, t40399, t40401, t40402, t40404) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1179(t59, t9223, t116, t120, t212, t22815, t67, t535, t1317, t40005, t12189, t3745);
        let (t40407, t40410, t40415, t40419) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1180(t1314, t9580, t3741, t2566, t3732, t12204, t12214, t792, t118, t12156, t794, t2229, t59, t60);
    (t40389, t40394, t40399, t40401, t40402, t40404, t40407, t40410, t40415, t40419)
}
