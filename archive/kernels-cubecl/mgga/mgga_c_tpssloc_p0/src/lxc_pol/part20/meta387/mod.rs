//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta387<F: Float>(t4282: F, t9632: F, t2732: F, t4234: F, t2679: F, t4295: F, t1519: F, t2627: F, t2633: F, t10076: F, t1510: F, t13381: F, t13385: F, t13388: F, t13390: F, t13393: F, t13397: F, t13398: F, t13401: F, t2617: F, t2729: F, t2733: F, t2736: F, t4166: F, t4281: F, t4291: F, t4292: F, t4296: F, t812: F) -> (F, F, F, F, F, F, F) {
        let (t13404, t13407, t13414, t13416, t13417, t13423, t13425) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1758::<F>(t4282, t9632, t2732, t4234, t2679, t4295, t1519, t2627, t2633, t10076, t1510, t13381, t13385, t13388, t13390, t13393, t13397, t13398, t13401, t2617, t2729, t2733, t2736, t4166, t4281, t4291, t4292, t4296, t812);
    (t13404, t13407, t13414, t13416, t13417, t13423, t13425)
}
