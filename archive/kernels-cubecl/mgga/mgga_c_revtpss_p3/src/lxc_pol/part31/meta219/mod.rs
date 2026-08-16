//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk980;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk981;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk982;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk983;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk984;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk985;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk986;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk987;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta219<F: Float>(t1427: F, t5774: F, t1424: F, t1445: F, t1904: F, t213: F, t3894: F, t3898: F, t3901: F, t3904: F, t3910: F, t3912: F, t3918: F, t3922: F, t4071: F, t5601: F, t5604: F, t561: F, t5711: F, t5715: F, t5719: F, t5723: F, t5728: F, t1343: F, t1353: F, t1448: F, t1450: F, t198: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t4139: F, t532: F, t5532: F, t5536: F, t5537: F, t5541: F, t5542: F, t5546: F, t5548: F, t5568: F, t5570: F, t5573: F, t5591: F, t5632: F, t1868: F, t4140: F, t3854: F, t3859: F, t3862: F, t3867: F, t3871: F, t3873: F, t4030: F, t4035: F, t4037: F, t4042: F, t5634: F, t5637: F, t5639: F, t5640: F, t5641: F, t118: F, t1310: F, t1315: F, t1453: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t2322: F, t4246: F, t4248: F, t4254: F, t4257: F, t4293: F, t4297: F, t508: F, t511: F, t5517: F, t5528: F, t569: F, t649: F, t651: F, t671: F, t3: F, param_d: F, t116: F, t1518: F, t670: F, t117: F, t4292: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, t2219: F, t2221: F, t2223: F, t2226: F, t2228: F, t2230: F, t2233: F, t2235: F, t2239: F, t1497: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5775 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk980::<F>(t1427, t5774);
        let t5778 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk981::<F>(t1424, t1445, t1904, t213, t3894, t3898, t3901, t3904, t3910, t3912, t3918, t3922, t4071, t5601, t5604, t561, t5711, t5715, t5719, t5723, t5728, t5775);
        let t5782 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk982::<F>(t1343, t1353, t1448, t1450, t198, t2522, t2562, t2569, t2579, t2587, t4139, t532, t5532, t5536, t5537, t5541, t5542, t5546, t5548, t5568, t5570, t5573, t5591, t5632, t5778);
        let t5786 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk983::<F>(t1868, t4140, t3854, t3859, t3862, t3867, t3871, t3873, t4030, t4035, t4037, t4042, t4139, t5634, t5637, t5639, t5640, t5641);
        let (t5787, t5789) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk984::<F>(t5782, t5786, t118, t1310, t1315, t1453, t1502, t1519, t1843, t1847, t1911, t2322, t4246, t4248, t4254, t4257, t4293, t4297, t508, t511, t5517, t5528, t569, t649, t651, t671);
        let (t5790, t5795) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk985::<F>(t3, t5789, param_d);
        let (t5801, t5802, t5805, t5808) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk986::<F>(t116, t1518, t670, t117, t4292, t1459, t1461, t1916, t1918, t572, t573, t5795);
        let (t5812, t5816) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk987::<F>(t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t2239, t1497);
    (t5775, t5778, t5787, t5789, t5790, t5795, t5801, t5802, t5805, t5808, t5812, t5816)
}
