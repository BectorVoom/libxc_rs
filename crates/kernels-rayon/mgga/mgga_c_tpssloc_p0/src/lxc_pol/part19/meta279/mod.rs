//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1043;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta279(t12132: f64, t17: f64, t3826: f64, t592: f64, t1285: f64, t2225: f64, t2371: f64, t3691: f64, t1294: f64, t9494: f64, t2535: f64, t12121: f64, t12123: f64, t12125: f64, t12128: f64, t12131: f64, t9853: f64, t9859: f64, t12049: f64, t12095: f64, t12119: f64, t225: f64, t1995: f64, t68: f64, t1307: f64, t3734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12133, t12135, t12137, t12139, t12141, t12143, t12144) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1043(t12132, t17, t3826, t592, t1285, t2225, t2371, t3691, t1294, t9494, t2535, t12121, t12123, t12125, t12128, t12131, t9853, t9859);
        let (t12147, t12155, t12156) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1044(t12049, t12095, t12119, t12144, t225, t1995, t68, t1307, t3734);
    (t12133, t12135, t12137, t12139, t12141, t12143, t12147, t12155, t12156)
}
