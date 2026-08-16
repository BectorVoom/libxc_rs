//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1891;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta595(t23788: f64, t86797: f64, t16596: f64, t83555: f64, t1081: f64, t4303: f64, t28: f64, t40772: f64, t86717: f64, t25365: f64, t1530: f64, t3231: f64, t1649: f64, t2749: f64, t57893: f64, t2752: f64, t13487: f64, t1390: f64, t16018: f64, t26062: f64, t645: f64, t72: f64, t26066: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t89928, t89931, t89941, t89954, t89972, t89978) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1891(t23788, t86797, t16596, t83555, t1081, t4303, t28, t40772, t86717, t25365, t1530, t3231);
        let (t89982, t89987, t89993, t90023, t90072, t90076) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1892(t1649, t2749, t23788, t57893, t2752, t13487, t1390, t16018, t26062, t645, t72, t26066);
    (t89928, t89931, t89941, t89954, t89972, t89978, t89982, t89987, t89993, t90023, t90072, t90076)
}
