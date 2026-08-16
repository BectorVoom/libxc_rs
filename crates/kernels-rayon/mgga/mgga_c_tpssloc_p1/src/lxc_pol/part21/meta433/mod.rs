//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1967;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1968;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta433(t15357: f64, t457: f64, t460: f64, t974: f64, t1716: f64, t698: f64, t1174: f64, t3435: f64, t4889: f64, t135: f64, t4930: f64, t1420: f64, t1887: f64, t337: f64, t11593: f64, t4904: f64, t11570: f64, t3961: f64, t11569: f64, t15332: f64, t15335: f64, t15341: f64, t3447: f64, t3452: f64, t3472: f64, t3478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15359, t15360, t15363, t15364, t15366, t15372, t15374, t15376) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1967(t15357, t457, t460, t974, t1716, t698, t1174, t3435, t4889, t135, t4930, t1420, t1887, t337);
        let (t15379, t15382) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1968(t11593, t4904, t11570, t3961);
        let (t15383, t15386) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1969(t11569, t15382, t1174, t15332, t15335, t15341, t15360, t15364, t15366, t15374, t15376, t15379, t3447, t3452, t3472, t3478, t4889);
    (t15359, t15360, t15363, t15364, t15366, t15372, t15374, t15376, t15379, t15382, t15383, t15386)
}
