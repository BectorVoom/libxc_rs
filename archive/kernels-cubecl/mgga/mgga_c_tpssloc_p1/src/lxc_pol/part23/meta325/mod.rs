//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1088;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta325<F: Float>(t22298: F, t475: F, t1214: F, t248: F, t11721: F, t3508: F, t11678: F, t11692: F, t11719: F, t11728: F, t11738: F, t15438: F, t15737: F, t15754: F, t1737: F, t1748: F, t19047: F, t19051: F, t19083: F, t19090: F, t19096: F, t22104: F, t22271: F, t22275: F, t22280: F, t22284: F, t22288: F, t3506: F, t3515: F, t3577: F, t467: F, t5005: F, t5024: F, t6207: F, t6211: F, t6227: F, t6232: F, t22152: F, t22202: F, t22267: F, t466: F, t1720: F, t6238: F, t1751: F, t6150: F, t1734: F, t1246: F, t491: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22299, t22301, t22307, t22309, t22312, t22314, t22325) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1088::<F>(t22298, t475, t1214, t248, t11721, t3508, t11678, t11692, t11719, t11728, t11738, t15438, t15737, t15754, t1737, t1748, t19047, t19051, t19083, t19090, t19096, t22104, t22271, t22275, t22280, t22284, t22288, t3506, t3515, t3577, t467, t5005, t5024, t6207, t6211, t6227, t6232);
        let (t22327, t22328, t22334, t22337, t22341, t22348) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1089::<F>(t22152, t22202, t22267, t22325, t466, t1720, t6238, t1751, t6150, t1734, t1246, t22298, t491);
    (t22299, t22301, t22307, t22309, t22312, t22314, t22327, t22328, t22334, t22337, t22341, t22348)
}
