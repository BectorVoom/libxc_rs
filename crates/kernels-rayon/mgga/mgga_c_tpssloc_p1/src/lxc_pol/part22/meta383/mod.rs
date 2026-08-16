//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1647;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta383(t14164: f64, t17686: f64, t4582: f64, t17691: f64, t4583: f64, t1023: f64, t17670: f64, t4594: f64, t17167: f64, t977: f64, t17171: f64, t17157: f64, t2979: f64, t5677: f64, t10408: f64, t1036: f64, t5905: f64, t1041: f64, t10876: f64, t10883: f64, t10952: f64, t13995: f64, t14158: f64, t14160: f64, t3070: f64, t3109: f64, t4579: f64, t5869: f64, t5880: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17971, t17972, t17975, t17976, t17979, t17980, t17983, t17984, t17988, t17991, t17994) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1647(t14164, t17686, t4582, t17691, t4583, t1023, t17670, t4594, t17167, t977, t17171, t17157, t2979);
        let (t17997, t17998, t18005, t18007) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1648(t1023, t5677, t10408, t1036, t5905, t1041, t10876, t10883, t10952, t13995, t14158, t14160, t17972, t17976, t17980, t17984, t17988, t17991, t17994, t3070, t3109, t4579, t5869, t5880, t973);
    (t17971, t17972, t17975, t17976, t17979, t17980, t17983, t17984, t17997, t17998, t18005, t18007)
}
