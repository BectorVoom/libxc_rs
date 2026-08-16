//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2231;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta619<F: Float>(t13133: F, t2655: F, t13123: F, t9885: F, t40738: F, t10140: F, t10143: F, t12971: F, t1484: F, t1530: F, t1877: F, t2522: F, t2523: F, t2749: F, t39483: F, t40741: F, t40743: F, t40772: F, t40785: F, t4255: F, t4303: F, t4314: F, t9470: F, t40745: F, t12908: F, t12924: F, t4101: F, t9912: F, t40754: F, t40761: F, t1409: F, t2516: F, t4194: F, t607: F, t40767: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46269, t46279, t46280, t46281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2231::<F>(t13133, t2655, t13123, t9885, t40738, t10140, t10143, t12971, t1484, t1530, t1877, t2522, t2523, t2749, t39483, t40741, t40743, t40772, t40785, t4255, t4303, t4314, t9470);
        let (t46282, t46284, t46286, t46287, t46288, t46292, t46293) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2232::<F>(t40745, t12908, t12924, t4101, t9912, t40754, t40761, t1409, t2516, t4194, t607, t40767);
    (t46269, t46279, t46280, t46281, t46282, t46284, t46286, t46287, t46288, t46292, t46293)
}
