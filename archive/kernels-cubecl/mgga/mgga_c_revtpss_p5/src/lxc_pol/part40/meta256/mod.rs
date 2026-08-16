//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta256 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk952;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk953;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk954;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk955;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta256<F: Float>(t213: F, t5744: F, t4086: F, t1892: F, t545: F, t869: F, t689: F, t72: F, t1432: F, t686: F, t1385: F, t1399: F, t1437: F, t1883: F, t4082: F, t4085: F, t4090: F, t4094: F, t4099: F, t4105: F, t4109: F, t4113: F, t4118: F, t546: F, t5659: F, t5675: F, t5710: F, t5735: F, t5738: F, t5742: F, t820: F, t1427: F, t1424: F, t1445: F, t1904: F, t3894: F, t3898: F, t3901: F, t3904: F, t3910: F, t3912: F, t3918: F, t3922: F, t4071: F, t5601: F, t5604: F, t561: F, t5711: F, t5715: F, t5719: F, t5723: F, t5728: F, t1343: F, t1353: F, t1448: F, t1450: F, t198: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t4139: F, t532: F, t5532: F, t5536: F, t5537: F, t5541: F, t5542: F, t5546: F, t5548: F, t5568: F, t5570: F, t5573: F, t5591: F, t5632: F, t1868: F, t4140: F, t3854: F, t3859: F, t3862: F, t3867: F, t3871: F, t3873: F, t4030: F, t4035: F, t4037: F, t4042: F, t5634: F, t5637: F, t5639: F, t5640: F, t5641: F, t118: F, t1310: F, t1315: F, t1453: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t2322: F, t4246: F, t4248: F, t4254: F, t4257: F, t4293: F, t4297: F, t508: F, t511: F, t5517: F, t5528: F, t569: F, t649: F, t651: F, t671: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5745, t5755, t5759, t5760, t5763, t5767, t5774) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk952::<F>(t213, t5744, t4086, t1892, t545, t869, t689, t72, t1432, t686, t1385, t1399, t1437, t1883, t4082, t4085, t4090, t4094, t4099, t4105, t4109, t4113, t4118, t546, t5659, t5675, t5710, t5735, t5738, t5742, t820);
        let (t5775, t5778) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk953::<F>(t1427, t5774, t1424, t1445, t1904, t213, t3894, t3898, t3901, t3904, t3910, t3912, t3918, t3922, t4071, t5601, t5604, t561, t5711, t5715, t5719, t5723, t5728);
        let t5782 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk954::<F>(t1343, t1353, t1448, t1450, t198, t2522, t2562, t2569, t2579, t2587, t4139, t532, t5532, t5536, t5537, t5541, t5542, t5546, t5548, t5568, t5570, t5573, t5591, t5632, t5778);
        let t5786 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk955::<F>(t1868, t4140, t3854, t3859, t3862, t3867, t3871, t3873, t4030, t4035, t4037, t4042, t4139, t5634, t5637, t5639, t5640, t5641);
        let (t5787, t5789) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk956::<F>(t5782, t5786, t118, t1310, t1315, t1453, t1502, t1519, t1843, t1847, t1911, t2322, t4246, t4248, t4254, t4257, t4293, t4297, t508, t511, t5517, t5528, t569, t649, t651, t671);
    (t5745, t5755, t5759, t5760, t5763, t5767, t5774, t5775, t5778, t5787, t5789)
}
