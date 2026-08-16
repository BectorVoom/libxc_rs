//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1782;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1783;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta439(t1834: f64, t5210: f64, t1807: f64, t5318: f64, t1842: f64, t5353: f64, t3887: f64, t1814: f64, t5333: f64, t1338: f64, t6434: f64, t1352: f64, t562: f64, t6414: f64, t5250: f64, t12171: f64, t6388: f64, t3901: f64, t6415: f64, t11984: f64, t15880: f64, t15889: f64, t15894: f64, t19543: f64, t19574: f64, t19576: f64, t19581: f64, t19588: f64, t19589: f64, t19590: f64, t19592: f64, t19594: f64, t9457: f64, t9476: f64, t9484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19635, t19644, t19648, t19654, t19657, t19658) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1782(t1834, t5210, t1807, t5318, t1842, t5353, t3887, t1814, t5333, t1338, t6434, t1352);
        let t19660 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1783(t562, t6414);
        let (t19661, t19668, t19674, t19676) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1784(t19660, t5250, t12171, t6388, t3901, t6415, t11984, t15880, t15889, t15894, t19543, t19574, t19576, t19581, t19588, t19589, t19590, t19592, t19594, t9457, t9476, t9484);
    (t19635, t19644, t19648, t19654, t19657, t19658, t19660, t19661, t19668, t19674, t19676)
}
