//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2061;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2062;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta533<F: Float>(t5854: F, t607: F, t10355: F, t5819: F, t606: F, t4186: F, t4201: F, t2275: F, t5825: F, t18281: F, t48: F, t10368: F, t4210: F, t2282: F, t60: F, t10379: F, t1480: F, t4211: F, t4214: F, t44: F, t56: F, t5835: F, t5838: F, t5843: F, t614: F, t620: F, t38: F, t10389: F, t2299: F, t10398: F, t2306: F, t4227: F, t4232: F, t633: F, t637: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21727, t21732, t21733, t21736, t21742, t21745, t21754) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2061::<F>(t5854, t607, t10355, t5819, t606, t4186, t4201, t2275, t5825, t18281, t48, t10368);
        let (t21755, t21758, t21762, t21765, t21768) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2062::<F>(t21754, t606, t4186, t4210, t2282, t5825, t18281, t60, t10379, t1480, t21733, t21736, t21742, t21745, t4211, t4214, t44, t56, t5835, t5838, t5843, t614, t620);
        let (t21769, t21784, t21794, t21804) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2063::<F>(t21768, t38, t10389, t5819, t2299, t5825, t10398, t2306, t18281, t4186, t4227, t4232, t606, t633, t637);
    (t21727, t21732, t21754, t21755, t21758, t21762, t21765, t21768, t21769, t21784, t21794, t21804)
}
