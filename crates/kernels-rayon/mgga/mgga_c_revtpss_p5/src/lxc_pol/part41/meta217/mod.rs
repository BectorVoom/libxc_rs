//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk842;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk843;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta217(t213: f64, t5744: f64, t4086: f64, t1892: f64, t545: f64, t869: f64, t689: f64, t72: f64, t1432: f64, t686: f64, t1385: f64, t1399: f64, t1437: f64, t1883: f64, t4082: f64, t4085: f64, t4090: f64, t4094: f64, t4099: f64, t4105: f64, t4109: f64, t4113: f64, t4118: f64, t546: f64, t5659: f64, t5675: f64, t5710: f64, t5735: f64, t5738: f64, t5742: f64, t820: f64, t1427: f64, t1424: f64, t1445: f64, t1904: f64, t3894: f64, t3898: f64, t3901: f64, t3904: f64, t3910: f64, t3912: f64, t3918: f64, t3922: f64, t4071: f64, t5601: f64, t5604: f64, t561: f64, t5711: f64, t5715: f64, t5719: f64, t5723: f64, t5728: f64, t1343: f64, t1353: f64, t1448: f64, t1450: f64, t198: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t4139: f64, t532: f64, t5532: f64, t5536: f64, t5537: f64, t5541: f64, t5542: f64, t5546: f64, t5548: f64, t5568: f64, t5570: f64, t5573: f64, t5591: f64, t5632: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767, t5774) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk842(t213, t5744, t4086, t1892, t545, t869, t689, t72, t1432, t686, t1385, t1399, t1437, t1883, t4082, t4085, t4090, t4094, t4099, t4105, t4109, t4113, t4118, t546, t5659, t5675, t5710, t5735, t5738, t5742, t820);
        let (t5775, t5778) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk843(t1427, t5774, t1424, t1445, t1904, t213, t3894, t3898, t3901, t3904, t3910, t3912, t3918, t3922, t4071, t5601, t5604, t561, t5711, t5715, t5719, t5723, t5728);
        let t5782 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk844(t1343, t1353, t1448, t1450, t198, t2522, t2562, t2569, t2579, t2587, t4139, t532, t5532, t5536, t5537, t5541, t5542, t5546, t5548, t5568, t5570, t5573, t5591, t5632, t5778);
    (t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767, t5774, t5775, t5778, t5782)
}
