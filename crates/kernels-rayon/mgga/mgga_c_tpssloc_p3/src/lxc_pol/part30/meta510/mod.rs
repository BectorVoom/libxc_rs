//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1832;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta510(t25742: f64, t3174: f64, t1054: f64, t1634: f64, t884: f64, t23329: f64, t225: f64, t7594: f64, t254: f64, t382: f64, t10164: f64, t1955: f64, t4664: f64, t1052: f64, t1066: f64, t14529: f64, t1635: f64, t1956: f64, t23327: f64, t23346: f64, t23359: f64, t23372: f64, t25447: f64, t25450: f64, t25453: f64, t25732: f64, t25736: f64, t25739: f64, t3026: f64, t6687: f64, t7557: f64, t7600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25743, t25749, t25750, t25751, t25755, t25757) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1832(t25742, t3174, t1054, t1634, t884, t23329, t225, t7594, t254, t382);
        let (t25758, t25759, t25762) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1833(t10164, t1955, t4664, t1052, t1066, t14529, t1635, t1956, t23327, t23346, t23359, t23372, t25447, t25450, t25453, t25732, t25736, t25739, t25743, t25751, t25755, t25757, t3026, t6687, t7557, t7600);
    (t25743, t25749, t25750, t25751, t25755, t25757, t25758, t25759, t25762)
}
