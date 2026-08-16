//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1157;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta271(t3: f64, t7945: f64, t1458: f64, t2039: f64, t1401: f64, t3941: f64, t5371: f64, t577: f64, t7230: f64, t7801: f64, t590: f64, t60: f64, t192: f64, t533: f64, t1390: f64, t2094: f64, t16: f64, t2: f64, t591: f64, t9: f64, t21: f64, t587: f64, t14: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7946, t7956, t7961, t8705) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1157(t3, t7945, t1458, t2039, t1401, t3941, t5371, t577, t7230, t7801, t590, t60);
        let (t8944, t9016, t9212, t9214, t9216, t9218) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1158(t192, t533, t1390, t2094, t16, t2, t591, t9, t21, t587, t14, t598);
    (t7946, t7956, t7961, t8705, t8944, t9016, t9212, t9214, t9216, t9218)
}
