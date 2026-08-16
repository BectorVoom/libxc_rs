//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta858 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3115;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta858(t14961: f64, t4869: f64, t18915: f64, t3415: f64, t14858: f64, t4875: f64, t15838: f64, t19267: f64, t3633: f64, t4700: f64, t63280: f64, t64446: f64, t64447: f64, t64454: f64, t64456: f64, t64458: f64, t64460: f64, t64462: f64, t64464: f64, t18918: f64, t3411: f64, t1703: f64, t51807: f64, t4879: f64, t15036: f64, t1155: f64, t4857: f64, t4861: f64, t51848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64466, t64470, t64472, t64473) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3115(t14961, t4869, t18915, t3415, t14858, t4875, t15838, t19267, t3633, t4700, t63280, t64446, t64447, t64454, t64456, t64458, t64460, t64462, t64464);
        let (t64475, t64477, t64479, t64481, t64482, t64485) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3116(t18918, t3411, t1703, t51807, t14858, t4879, t15036, t4869, t1155, t4857, t4861, t51848);
    (t64466, t64470, t64472, t64473, t64475, t64477, t64479, t64481, t64482, t64485)
}
