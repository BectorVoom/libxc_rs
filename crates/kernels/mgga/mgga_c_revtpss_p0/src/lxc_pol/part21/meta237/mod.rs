//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1396;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta237<F: Float>(t3: F, t5789: F, t116: F, t1518: F, t670: F, t117: F, t4292: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, param_d: F, t159: F, t793: F, t1448: F, t4147: F, t1493: F, t76: F, t587: F, t65: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5790, t5795, t5801, t5802, t5805, t5808) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1396::<F>(t3, t5789, t116, t1518, t670, t117, t4292, t1459, t1461, t1916, t1918, t572, t573, param_d);
        let (t7021, t7315, t7719, t8779) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1397::<F>(t159, t793, t1448, t4147, t1493, t76, t587, t65);
    (t5790, t5795, t5801, t5802, t5805, t5808, t7021, t7315, t7719, t8779)
}
