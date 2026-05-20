//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2186;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2187;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2188;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2189;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta534<F: Float>(t3367: F, t4186: F, t606: F, t1120: F, t128: F, t2258: F, t5051: F, t1121: F, t13312: F, t12297: F, t12299: F, t12301: F, t12303: F, t12610: F, t16706: F, t16708: F, t16711: F, t16713: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16737, t16738, t16739, t16740) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2186::<F>(t3367, t4186, t606, t1120, t128);
        let (t16742, t16743, t16744) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2187::<F>(t2258, t5051, t1120, t128);
        let t16746 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2188::<F>(t1121, t13312);
        let (t16747, t16748) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2189::<F>(t1120, t16746, t128);
        let t16750 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2190::<F>(t12297, t12299, t12301, t12303, t12610, t16706, t16708, t16711, t16713, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16737, t16738, t16739, t16740, t16742, t16743, t16744, t16746, t16747, t16748, t16750)
}
