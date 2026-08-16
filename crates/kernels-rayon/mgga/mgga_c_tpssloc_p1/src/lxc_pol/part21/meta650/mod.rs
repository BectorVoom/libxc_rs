//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2445;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta650(t10189: f64, t3008: f64, t4509: f64, t13797: f64, t984: f64, t10216: f64, t343: f64, t3152: f64, t698: f64, t973: f64, t10870: f64, t3117: f64, t2955: f64, t3158: f64, t10383: f64, t964: f64, t1020: f64, t10508: f64, t248: f64, t3121: f64, t10868: f64, t820: f64, t3070: f64, t3072: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43057, t43065, t43069, t43070, t43110, t43114) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2445(t10189, t3008, t4509, t13797, t984, t10216, t343, t3152, t698, t973, t10870, t3117);
        let (t43155, t43157, t43161, t43198, t43200) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2446(t2955, t3158, t10383, t964, t1020, t10508, t248, t3121, t10868, t820, t3070, t3072);
    (t43057, t43065, t43069, t43070, t43110, t43114, t43155, t43157, t43161, t43198, t43200)
}
