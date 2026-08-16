//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1161;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta362<F: Float>(t204: F, t376: F, t370: F, t374: F, t9697: F, t10473: F, t361: F, t363: F, t42342: F, t42345: F, t3131: F, t221: F, t339: F, t42813: F, t10216: F, t2978: F, t3061: F, t676: F, t11065: F, t42387: F, t10475: F, t2770: F, t283: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43216, t43253, t43288, t43291, t43292, t43307) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1161::<F>(t204, t376, t370, t374, t9697, t10473, t361, t363, t42342, t42345, t3131, t221, t339, t42813);
        let (t43317, t43338, t43361, t43385, t43399) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1162::<F>(t10216, t2978, t3061, t676, t11065, t42387, t10475, t42342, t42345, t2770, t283, t61);
    (t43216, t43253, t43288, t43291, t43292, t43307, t43317, t43338, t43361, t43385, t43399)
}
