//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2561;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta678(t11629: f64, t4869: f64, t14967: f64, t3411: f64, t51474: f64, t51476: f64, t51478: f64, t51480: f64, t51482: f64, t51485: f64, t51549: f64, t51593: f64, t51831: f64, t11366: f64, t1164: f64, t14853: f64, t11129: f64, t1694: f64, t43689: f64, t43692: f64, t11400: f64, t4874: f64, t11365: f64, t300: f64, t4861: f64, t51811: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t51833, t51835, t51836) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2561(t11629, t4869, t14967, t3411, t51474, t51476, t51478, t51480, t51482, t51485, t51549, t51593, t51831);
        let (t51839, t51844, t51847, t51851) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2562(t11366, t1164, t14853, t11129, t1694, t43689, t43692, t11400, t4874, t11365, t300, t4861, t51811);
    (t51833, t51835, t51836, t51839, t51844, t51847, t51851)
}
