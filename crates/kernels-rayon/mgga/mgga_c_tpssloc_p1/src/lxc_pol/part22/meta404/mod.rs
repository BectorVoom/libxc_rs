//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1702;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta404(t11569: f64, t18469: f64, t1180: f64, t15284: f64, t15287: f64, t15300: f64, t15307: f64, t18321: f64, t18443: f64, t18447: f64, t18452: f64, t18455: f64, t18458: f64, t18460: f64, t18466: f64, t3447: f64, t4889: f64, t4937: f64, t18211: f64, t4900: f64, t15382: f64, t15390: f64, t1171: f64, t6109: f64, t6011: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t18470, t18473) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1702(t11569, t18469, t1180, t15284, t15287, t15300, t15307, t18321, t18443, t18447, t18452, t18455, t18458, t18460, t18466, t3447, t4889, t4937);
        let (t18475, t18484, t18489, t18494) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1703(t18211, t4900, t15382, t15390, t1171, t6109, t6011, t699);
    (t18470, t18473, t18475, t18484, t18489, t18494)
}
