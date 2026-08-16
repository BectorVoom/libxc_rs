//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2176;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta638<F: Float>(t19572: F, t67: F, t758: F, t2221: F, t6328: F, t2225: F, t17: F, t2516: F, t6320: F, t750: F, t19644: F, t225: F, t20038: F, t212: F, t6330: F, t2586: F, t40353: F, t6347: F, t12225: F, t118: F, t19631: F, t3739: F, t794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t56374, t56390, t56394, t56398, t56400, t56422) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2176::<F>(t19572, t67, t758, t2221, t6328, t2225, t17, t2516, t6320, t750, t19644, t225);
        let (t56434, t56463, t56465, t56467, t56469, t56482) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2177::<F>(t20038, t225, t212, t6330, t2586, t40353, t6347, t12225, t118, t19631, t3739, t794);
    (t56374, t56390, t56394, t56398, t56400, t56422, t56434, t56463, t56465, t56467, t56469, t56482)
}
