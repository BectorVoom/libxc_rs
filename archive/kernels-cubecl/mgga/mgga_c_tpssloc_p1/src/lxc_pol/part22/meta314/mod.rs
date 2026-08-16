//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1492;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta314<F: Float>(t11570: F, t3961: F, t1714: F, t4899: F, t11545: F, t60: F, t461: F, t11589: F, t4904: F, t3447: F, t11588: F, t4729: F, t134: F, t3439: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15382, t15390, t15394, t15395, t15399, t15401, t15402) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1492::<F>(t11570, t3961, t1714, t4899, t11545, t60, t461, t11589, t4904, t3447, t11588);
        let (t15403, t15405, t15418, t15419) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1493::<F>(t15402, t4729, t3447, t134, t3439, t461);
    (t15382, t15390, t15394, t15395, t15399, t15401, t15402, t15403, t15405, t15418, t15419)
}
