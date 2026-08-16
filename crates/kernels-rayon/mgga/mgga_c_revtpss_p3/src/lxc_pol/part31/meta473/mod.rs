//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1736;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1737;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1738;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta473(t4147: f64, t6922: f64, t566: f64, t6816: f64, t1448: f64, t1868: f64, t1353: f64, t13664: f64, t13682: f64, t13683: f64, t198: f64, t22214: f64, t22215: f64, t22216: f64, t22217: f64, t22218: f64, t22219: f64, t4139: f64, t4140: f64, t5536: f64, t5541: f64, t5542: f64, t5778: f64, t6836: f64, t9524: f64, t9542: f64, t9854: f64, t9865: f64, t9868: f64, t22465: f64, t22473: f64, t22482: f64, t1312: f64, t13426: f64, t1518: f64, t18220: f64, t18227: f64, t18245: f64, t21814: f64, t21881: f64, t2322: f64, t4248: f64, t4292: f64, t5523: f64, t5920: f64, t670: f64, t7889: f64, t1315: f64, t1453: f64, t1847: f64, t1911: f64, t21882: f64, t21891: f64, t4254: f64, t4293: f64, t4297: f64, t508: f64, t511: f64, t5528: f64, t569: f64, t5787: f64, t5887: f64, t649: f64, t651: f64, t6765: f64, t6773: f64, t6934: f64, t7732: f64, t21660: f64, t3: f64, t5883: f64, t5801: f64, t116: f64, t117: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t6941: f64, t6945: f64, t6948: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22483, t22496, t22504) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1736(t4147, t6922, t566, t6816, t1448, t1868, t1353, t13664, t13682, t13683, t198, t22214, t22215, t22216, t22217, t22218, t22219, t4139, t4140, t5536, t5541, t5542, t5778, t6836, t9524, t9542, t9854, t9865, t9868);
        let (t22506, t22525) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1737(t22465, t22473, t22482, t22504, t1312, t13426, t1518, t18220, t18227, t18245, t21814, t21881, t2322, t4248, t4292, t5523, t5920, t670, t7889);
        let t22531 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1738(t1315, t1453, t1847, t1911, t21814, t21882, t21891, t22506, t22525, t2322, t4248, t4254, t4293, t4297, t508, t511, t5528, t569, t5787, t5887, t649, t651, t6765, t6773, t6934, t7732);
        let (t22533, t22544, t22556, t22559, t22565, t22568, t22571) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1739(t21660, t22531, t3, t5883, t670, t4292, t5801, t116, t5920, t117, t21881, t1459, t1461, t1916, t1918, t572, t573, t5795, t5802, t5805, t6941, t6945, t6948, param_d);
    (t22483, t22496, t22506, t22525, t22533, t22544, t22556, t22559, t22565, t22568, t22571)
}
