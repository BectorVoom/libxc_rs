//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1901;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1902;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1903;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta636(t1985: f64, t20009: f64, t214: f64, t225: f64, t567: f64, t3886: f64, t6439: f64, t1307: f64, t22633: f64, t22635: f64, t26193: f64, t26202: f64, t6888: f64, t6891: f64, t97511: f64, t28116: f64, t80650: f64, t1808: f64, t254: f64, t1377: f64, t6347: f64, t1385: f64, t1842: f64, t90516: f64, t1992: f64, t26355: f64, t90566: f64, t26331: f64, t20022: f64, t6889: f64, t6906: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97604, t97611, t97616) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1901(t1985, t20009, t214, t225, t567, t3886, t6439, t1307, t22633, t22635, t26193, t26202);
        let (t97619, t97624, t97626, t97640) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1902(t6888, t6891, t97511, t22633, t28116, t80650, t1808, t254, t1377, t6347, t1385, t22635);
        let (t97644, t97647, t97652, t97658) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1903(t1842, t22633, t22635, t90516, t1992, t26355, t90566, t1307, t26331, t567, t6347, t1985, t20022, t6889, t6906);
    (t97604, t97611, t97616, t97619, t97624, t97626, t97640, t97644, t97647, t97652, t97658)
}
