//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2231;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta619(t13133: f64, t2655: f64, t13123: f64, t9885: f64, t40738: f64, t10140: f64, t10143: f64, t12971: f64, t1484: f64, t1530: f64, t1877: f64, t2522: f64, t2523: f64, t2749: f64, t39483: f64, t40741: f64, t40743: f64, t40772: f64, t40785: f64, t4255: f64, t4303: f64, t4314: f64, t9470: f64, t40745: f64, t12908: f64, t12924: f64, t4101: f64, t9912: f64, t40754: f64, t40761: f64, t1409: f64, t2516: f64, t4194: f64, t607: f64, t40767: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46269, t46279, t46280, t46281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2231(t13133, t2655, t13123, t9885, t40738, t10140, t10143, t12971, t1484, t1530, t1877, t2522, t2523, t2749, t39483, t40741, t40743, t40772, t40785, t4255, t4303, t4314, t9470);
        let (t46282, t46284, t46286, t46287, t46288, t46292, t46293) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2232(t40745, t12908, t12924, t4101, t9912, t40754, t40761, t1409, t2516, t4194, t607, t40767);
    (t46269, t46279, t46280, t46281, t46282, t46284, t46286, t46287, t46288, t46292, t46293)
}
