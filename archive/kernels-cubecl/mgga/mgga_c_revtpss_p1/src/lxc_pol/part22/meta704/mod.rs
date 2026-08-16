//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2721;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2722;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta704<F: Float>(t1444: F, t6895: F, t9657: F, t22307: F, t225: F, t212: F, t6888: F, t1358: F, t689: F, t1357: F, t6896: F, t72: F, t686: F, t9680: F, t10160: F, t10163: F, t10166: F, t1424: F, t14280: F, t14290: F, t14294: F, t14297: F, t213: F, t4071: F, t561: F, t6919: F, t22393: F, t22418: F, t22430: F, t1343: F, t1353: F, t13599: F, t13600: F, t1450: F, t1868: F, t198: F, t21901: F, t21905: F, t21933: F, t21937: F, t21969: F, t4139: F, t532: F, t5532: F, t5536: F, t5591: F, t5627: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22432, t22433, t22441, t22445, t22446, t22447, t22449, t22450, t22452) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2721::<F>(t1444, t6895, t9657, t22307, t225, t212, t6888, t1358, t689, t1357, t6896, t72);
        let (t22453, t22459) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2722::<F>(t22452, t686, t9680, t10160, t10163, t10166, t1424, t14280, t14290, t14294, t14297, t213, t22433, t22441, t22447, t22450, t4071, t561, t6919);
        let (t22461, t22465) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2723::<F>(t22393, t22418, t22430, t22459, t1343, t1353, t13599, t13600, t1450, t1868, t198, t21901, t21905, t21933, t21937, t21969, t4139, t532, t5532, t5536, t5591, t5627, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
    (t22432, t22433, t22441, t22445, t22446, t22449, t22452, t22453, t22461, t22465)
}
