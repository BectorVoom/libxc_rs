//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta417(t1118: f64, t14913: f64, t1099: f64, t14720: f64, t14722: f64, t14704: f64, t11136: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14702: f64, t14708: f64, t14728: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t14914, t14916, t14922, t14923, t14924, t14933) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1935(t1118, t14913, t1099, t14720, t14722, t14704, t11136, t11137, t11139, t11141, t11143, t14702, t14708, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
    (t14914, t14916, t14922, t14923, t14924, t14933)
}
