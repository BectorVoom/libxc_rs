//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1743;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1744;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1745;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta477<F: Float>(t4147: F, t6922: F, t566: F, t6816: F, t1448: F, t1868: F, t1353: F, t13664: F, t13682: F, t13683: F, t198: F, t22214: F, t22215: F, t22216: F, t22217: F, t22218: F, t22219: F, t4139: F, t4140: F, t5536: F, t5541: F, t5542: F, t5778: F, t6836: F, t9524: F, t9542: F, t9854: F, t9865: F, t9868: F, t22465: F, t22473: F, t22482: F, t1312: F, t13426: F, t1518: F, t18220: F, t18227: F, t18245: F, t21814: F, t21881: F, t2322: F, t4248: F, t4292: F, t5523: F, t5920: F, t670: F, t7889: F, t1315: F, t1453: F, t1847: F, t1911: F, t21882: F, t21891: F, t4254: F, t4293: F, t4297: F, t508: F, t511: F, t5528: F, t569: F, t5787: F, t5887: F, t649: F, t651: F, t6765: F, t6773: F, t6934: F, t7732: F, t21660: F, t3: F, t5883: F, t5801: F, t116: F, t117: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t6941: F, t6945: F, t6948: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22483, t22496, t22504) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1743::<F>(t4147, t6922, t566, t6816, t1448, t1868, t1353, t13664, t13682, t13683, t198, t22214, t22215, t22216, t22217, t22218, t22219, t4139, t4140, t5536, t5541, t5542, t5778, t6836, t9524, t9542, t9854, t9865, t9868);
        let (t22506, t22525) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1744::<F>(t22465, t22473, t22482, t22504, t1312, t13426, t1518, t18220, t18227, t18245, t21814, t21881, t2322, t4248, t4292, t5523, t5920, t670, t7889);
        let t22531 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1745::<F>(t1315, t1453, t1847, t1911, t21814, t21882, t21891, t22506, t22525, t2322, t4248, t4254, t4293, t4297, t508, t511, t5528, t569, t5787, t5887, t649, t651, t6765, t6773, t6934, t7732);
        let (t22533, t22544, t22556, t22559, t22565, t22568, t22571) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1746::<F>(t21660, t22531, t3, t5883, t670, t4292, t5801, t116, t5920, t117, t21881, t1459, t1461, t1916, t1918, t572, t573, t5795, t5802, t5805, t6941, t6945, t6948, param_d);
    (t22483, t22496, t22506, t22525, t22533, t22544, t22556, t22559, t22565, t22568, t22571)
}
