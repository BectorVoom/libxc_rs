//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1955;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta560<F: Float>(t30100: F, t7301: F, t1882: F, t1903: F, t543: F, t25931: F, t2030: F, t213: F, t25930: F, t26040: F, t26043: F, t26058: F, t26071: F, t27837: F, t27966: F, t27969: F, t27987: F, t27990: F, t27992: F, t30071: F, t30074: F, t30082: F, t30089: F, t30096: F, t561: F, t6896: F, t7279: F, t7295: F, t7917: F, t7926: F, t7930: F, t30066: F, t532: F, t1450: F, t2014: F, t1868: F, t1907: F, t8717: F, t25082: F, t7732: F, t7742: F, t1936: F, t6765: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t30101, t30105, t30106, t30109) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1955::<F>(t30100, t7301, t1882, t1903, t543, t25931, t2030, t213, t25930, t26040, t26043, t26058, t26071, t27837, t27966, t27969, t27987, t27990, t27992, t30071, t30074, t30082, t30089, t30096, t561, t6896, t7279, t7295, t7917, t7926, t7930);
        let (t30110, t30111, t30112, t30113, t30122, t30123, t30125, t30127, t30128) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1956::<F>(t30066, t30109, t532, t1450, t2014, t1868, t1907, t8717, t25082, t7732, t7742, t1936, t6765);
    (t30101, t30105, t30106, t30110, t30111, t30112, t30113, t30122, t30123, t30125, t30127, t30128)
}
