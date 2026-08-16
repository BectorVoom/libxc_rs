//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta513 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1809;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1810;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1811;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1812;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta513<F: Float>(t30: F, t265: F, t393: F, t30462: F, t1469: F, t2078: F, t30438: F, t45: F, t5825: F, t8040: F, t2071: F, t29939: F, t1711: F, t1940: F, t2403: F, t26425: F, t26590: F, t28460: F, t29946: F, t29949: F, t29953: F, t29964: F, t29967: F, t29970: F, t30420: F, t33: F, t4541: F, t6416: F, t7432: F, t7862: F, t7869: F, t8020: F, dens_threshold: F, rho0: F, zeta_threshold: F, t502: F, t2085: F, t57: F, t8059: F, t26405: F, t30122: F, t2047: F, t29532: F, rho1: F, t1923: F, t2048: F, t26175: F, t26207: F, t28154: F, t28598: F, t28600: F, t28602: F, t28628: F, t28638: F, t28641: F, t29513: F, t29538: F, t29544: F, t29548: F, t29551: F, t29554: F, t29562: F, t7343: F, t7702: F, t7706: F, t7709: F, t7964: F, t5: F, t117: F, t118: F, t18245: F, t1911: F, t2014: F, t2056: F, t2093: F, t2108: F, t25082: F, t29506: F, t29508: F, t30138: F, t30209: F, t30218: F, t30315: F, t4248: F, t508: F, t5887: F, t651: F, t6934: F, t7359: F, t7732: F, t7898: F, t7978: F, t7984: F, t8075: F, t8079: F, t8109: F, t8111: F, t114: F, t2089: F, t5920: F, t2055: F, t6765: F, t26148: F, t28034: F, t29999: F, t30001: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t30463, t30470, t30471, t30502) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1809::<F>(t30, t265, t393, t30462, t1469, t2078, t30438, t45, t5825, t8040, t2071, t29939, t1711, t1940, t2403, t26425, t26590, t28460, t29946, t29949, t29953, t29964, t29967, t29970, t30420, t33, t4541, t6416, t7432, t7862, t7869, t8020, dens_threshold, rho0, zeta_threshold);
        let (t30503, t30511, t30513, t30543) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1810::<F>(t33, t265, t502, t30462, t1469, t2085, t30502, t57, t5825, t8059, t30470, t26405, t30122, t2047, t29532, dens_threshold, rho1, zeta_threshold);
        let t30551 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1811::<F>(t1923, t2048, t26175, t26207, t28154, t28598, t28600, t28602, t28628, t28638, t28641, t29513, t29538, t29544, t29548, t29551, t29554, t29562, t30543, t7343, t7702, t7706, t7709, t7964);
        let (t30552, t30553, t30555) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1812::<F>(t5, t30551, t117, t118, t18245, t1911, t2014, t2056, t2093, t2108, t25082, t29506, t29508, t30138, t30209, t30218, t30315, t30511, t30513, t4248, t508, t5887, t651, t6934, t7359, t7732, t7898, t7978, t7984, t8075, t8079, t8109, t8111);
        let (t30558, t30563, t30570) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1813::<F>(t114, t2089, t5920, t2055, t6765, t26148, t28034, t29999, t30001);
    (t30463, t30471, t30503, t30511, t30513, t30543, t30552, t30553, t30555, t30558, t30563, t30570)
}
