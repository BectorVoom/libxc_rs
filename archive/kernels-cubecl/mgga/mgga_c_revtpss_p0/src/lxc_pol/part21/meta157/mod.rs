//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1002;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1003;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta157<F: Float>(t3670: F, t480: F, t3568: F, t482: F, t371: F, t372: F, t1236: F, t127: F, t1235: F, t221: F, t462: F, t696: F, t461: F, t1226: F, t140: F, t1222: F, t1225: F, t2258: F, t1012: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3671, t3672, t3674) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1002::<F>(t3670, t480, t3568, t482, t371, t372);
        let t3678 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1003::<F>(t1236, t127, t371);
        let (t3679, t3682, t3684, t3685, t3686, t3688, t3689) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1004::<F>(t1235, t3678, t221, t462, t696, t461, t1226, t140, t1222, t1225, t2258, t1012);
    (t3671, t3672, t3674, t3678, t3679, t3682, t3684, t3685, t3686, t3688, t3689)
}
