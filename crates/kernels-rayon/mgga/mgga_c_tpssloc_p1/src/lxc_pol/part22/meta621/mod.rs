//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2153;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta621(t53033: f64, t1213: f64, t1735: f64, t248: f64, t45017: f64, t10477: f64, t1742: f64, t11713: f64, t3503: f64, t1210: f64, t11529: f64, t1174: f64, t4729: f64, t11647: f64, t1731: f64, t3577: f64, t44951: f64, t4949: f64, t3242: f64, t3448: f64, t11718: f64, t52835: f64, t11147: f64, t15394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53034, t53079, t53081, t53083, t53087, t53096) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2153(t53033, t1213, t1735, t248, t45017, t10477, t1742, t11713, t3503, t1210, t11529, t1174, t4729);
        let (t53097, t53099, t53162, t53187, t53238, t53249) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2154(t53096, t11647, t1731, t3577, t44951, t4949, t3242, t3448, t11718, t52835, t11147, t15394);
    (t53034, t53079, t53081, t53083, t53087, t53097, t53099, t53162, t53187, t53238, t53249)
}
