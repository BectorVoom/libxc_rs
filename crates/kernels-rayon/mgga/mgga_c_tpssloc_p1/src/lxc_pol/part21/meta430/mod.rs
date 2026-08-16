//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1962;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta430(t15293: f64, t3449: f64, t11529: f64, t1709: f64, t1174: f64, t1714: f64, t3475: f64, t460: f64, t4934: f64, t3432: f64, t4889: f64, t3450: f64, t3966: f64, t14749: f64, t4908: f64, t3448: f64, t4928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15294, t15299, t15300, t15303, t15304, t15307, t15313) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1962(t15293, t3449, t11529, t1709, t1174, t1714, t3475, t460, t4934, t3432, t4889, t3450, t3966);
        let (t15314, t15317, t15320) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1963(t15313, t3449, t14749, t4908, t3448, t4928);
    (t15294, t15299, t15300, t15303, t15304, t15307, t15313, t15314, t15317, t15320)
}
