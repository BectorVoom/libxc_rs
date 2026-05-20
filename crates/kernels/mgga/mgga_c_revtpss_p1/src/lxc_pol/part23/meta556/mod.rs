//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2114;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta556<F: Float>(t1357: F, t6919: F, t689: F, t1444: F, t6918: F, t4076: F, t14081: F, t14084: F, t14087: F, t1424: F, t14299: F, t1904: F, t22395: F, t22400: F, t22405: F, t22407: F, t9677: F, t9687: F, t9691: F, t5599: F, t10157: F, t14091: F, t14096: F, t14097: F, t14102: F, t14105: F, t14108: F, t14111: F, t14276: F, t5715: F, t5728: F, t9694: F, t9695: F) -> (F, F, F, F, F, F, F, F) {
        let (t22409, t22410, t22414, t22415, t22418) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2114::<F>(t1357, t6919, t689, t1444, t6918, t4076, t14081, t14084, t14087, t1424, t14299, t1904, t22395, t22400, t22405, t22407, t9677, t9687, t9691);
        let (t22427, t22428, t22430) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2115::<F>(t1904, t5599, t689, t10157, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t14276, t5715, t5728, t9694, t9695);
    (t22409, t22410, t22414, t22415, t22418, t22427, t22428, t22430)
}
