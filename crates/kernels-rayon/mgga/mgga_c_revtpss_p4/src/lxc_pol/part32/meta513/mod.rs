//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1809;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1810;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1811;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1812;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta513(t30: f64, t265: f64, t393: f64, t30462: f64, t1469: f64, t2078: f64, t30438: f64, t45: f64, t5825: f64, t8040: f64, t2071: f64, t29939: f64, t1711: f64, t1940: f64, t2403: f64, t26425: f64, t26590: f64, t28460: f64, t29946: f64, t29949: f64, t29953: f64, t29964: f64, t29967: f64, t29970: f64, t30420: f64, t33: f64, t4541: f64, t6416: f64, t7432: f64, t7862: f64, t7869: f64, t8020: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t502: f64, t2085: f64, t57: f64, t8059: f64, t26405: f64, t30122: f64, t2047: f64, t29532: f64, rho1: f64, t1923: f64, t2048: f64, t26175: f64, t26207: f64, t28154: f64, t28598: f64, t28600: f64, t28602: f64, t28628: f64, t28638: f64, t28641: f64, t29513: f64, t29538: f64, t29544: f64, t29548: f64, t29551: f64, t29554: f64, t29562: f64, t7343: f64, t7702: f64, t7706: f64, t7709: f64, t7964: f64, t5: f64, t117: f64, t118: f64, t18245: f64, t1911: f64, t2014: f64, t2056: f64, t2093: f64, t2108: f64, t25082: f64, t29506: f64, t29508: f64, t30138: f64, t30209: f64, t30218: f64, t30315: f64, t4248: f64, t508: f64, t5887: f64, t651: f64, t6934: f64, t7359: f64, t7732: f64, t7898: f64, t7978: f64, t7984: f64, t8075: f64, t8079: f64, t8109: f64, t8111: f64, t114: f64, t2089: f64, t5920: f64, t2055: f64, t6765: f64, t26148: f64, t28034: f64, t29999: f64, t30001: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30463, t30470, t30471, t30502) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1809(t30, t265, t393, t30462, t1469, t2078, t30438, t45, t5825, t8040, t2071, t29939, t1711, t1940, t2403, t26425, t26590, t28460, t29946, t29949, t29953, t29964, t29967, t29970, t30420, t33, t4541, t6416, t7432, t7862, t7869, t8020, dens_threshold, rho0, zeta_threshold);
        let (t30503, t30511, t30513, t30543) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1810(t33, t265, t502, t30462, t1469, t2085, t30502, t57, t5825, t8059, t30470, t26405, t30122, t2047, t29532, dens_threshold, rho1, zeta_threshold);
        let t30551 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1811(t1923, t2048, t26175, t26207, t28154, t28598, t28600, t28602, t28628, t28638, t28641, t29513, t29538, t29544, t29548, t29551, t29554, t29562, t30543, t7343, t7702, t7706, t7709, t7964);
        let (t30552, t30553, t30555) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1812(t5, t30551, t117, t118, t18245, t1911, t2014, t2056, t2093, t2108, t25082, t29506, t29508, t30138, t30209, t30218, t30315, t30511, t30513, t4248, t508, t5887, t651, t6934, t7359, t7732, t7898, t7978, t7984, t8075, t8079, t8109, t8111);
        let (t30558, t30563, t30570) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1813(t114, t2089, t5920, t2055, t6765, t26148, t28034, t29999, t30001);
    (t30463, t30471, t30503, t30511, t30513, t30543, t30552, t30553, t30555, t30558, t30563, t30570)
}
