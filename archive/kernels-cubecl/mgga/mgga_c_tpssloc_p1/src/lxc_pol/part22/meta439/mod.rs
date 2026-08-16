//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1782;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1783;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta439<F: Float>(t1834: F, t5210: F, t1807: F, t5318: F, t1842: F, t5353: F, t3887: F, t1814: F, t5333: F, t1338: F, t6434: F, t1352: F, t562: F, t6414: F, t5250: F, t12171: F, t6388: F, t3901: F, t6415: F, t11984: F, t15880: F, t15889: F, t15894: F, t19543: F, t19574: F, t19576: F, t19581: F, t19588: F, t19589: F, t19590: F, t19592: F, t19594: F, t9457: F, t9476: F, t9484: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19635, t19644, t19648, t19654, t19657, t19658) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1782::<F>(t1834, t5210, t1807, t5318, t1842, t5353, t3887, t1814, t5333, t1338, t6434, t1352);
        let t19660 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1783::<F>(t562, t6414);
        let (t19661, t19668, t19674, t19676) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1784::<F>(t19660, t5250, t12171, t6388, t3901, t6415, t11984, t15880, t15889, t15894, t19543, t19574, t19576, t19581, t19588, t19589, t19590, t19592, t19594, t9457, t9476, t9484);
    (t19635, t19644, t19648, t19654, t19657, t19658, t19660, t19661, t19668, t19674, t19676)
}
