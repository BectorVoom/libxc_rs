//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1957;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta507(t136: f64, t21801: f64, t11243: f64, t21785: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t21781: f64, t21783: f64, t21786: f64, t21789: f64, t21792: f64, t21795: f64, t21753: f64, t1118: f64, t1099: f64, t11277: f64, t21723: f64, t11275: f64, t11136: f64, t14702: f64, t18203: f64, t18219: f64, t18229: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21802, t21804, t21808) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1957(t136, t21801, t11243, t21785, t21760, t21764, t21767, t21771, t21774, t21778, t21781, t21783, t21786, t21789, t21792, t21795);
        let (t21809, t21810, t21812, t21813, t21815, t21826) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1958(t21753, t21808, t1118, t1099, t11277, t21723, t11275, t11136, t14702, t18203, t18219, t18229, t21760, t21764, t21767, t21771, t21774, t21778);
    (t21802, t21804, t21809, t21810, t21812, t21813, t21815, t21826)
}
