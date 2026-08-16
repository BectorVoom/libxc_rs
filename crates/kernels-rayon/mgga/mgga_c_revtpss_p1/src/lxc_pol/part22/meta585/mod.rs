//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2451;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2452;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2453;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta585(t6017: f64, t72: f64, t686: f64, t2798: f64, t5978: f64, t14568: f64, t4500: f64, t18699: f64, t231: f64, t2783: f64, t2782: f64, t18677: f64, t18681: f64, t2723: f64, t4503: f64, t10916: f64, t14577: f64, t14581: f64, t14590: f64, t14596: f64, t14603: f64, t14608: f64, t14948: f64, t6041: f64, t874: f64, t10661: f64, t10923: f64, t10925: f64, t10939: f64, t10948: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t14546: f64, t14951: f64, t14972: f64, t1559: f64, t18525: f64, t4366: f64, t4504: f64, t6022: f64, t820: f64, t18687: f64, t18722: f64, t868: f64, t10503: f64, t10507: f64, t10511: f64, t10984: f64, t14998: f64, t15004: f64, t15006: f64, t15010: f64, t15015: f64, t18324: f64, t18658: f64, t18663: f64, t213: f64, t257: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18725, t18726, t18727, t18729, t18730, t18731, t18733, t18738, t18739, t18742) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2451(t6017, t72, t686, t2798, t5978, t14568, t4500, t18699, t231, t2783, t2782, t18677);
        let (t18746, t18750, t18754) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2452(t18742, t2782, t18681, t231, t2783, t18677, t2723, t4503, t10916, t14577, t14581, t14590, t14596, t14603, t14608, t14948, t18727, t18731, t18733, t18739);
        let (t18761, t18782) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2453(t6041, t72, t686, t874, t10661, t10923, t10925, t10939, t10948, t10964, t10966, t10969, t10971, t14546, t14951, t14972, t1559, t18525, t18677, t18681, t18699, t4366, t4504, t6022, t820);
        let (t18784, t18785, t18791) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2454(t18687, t18722, t18754, t18782, t868, t10503, t10507, t10511, t10984, t14998, t15004, t15006, t15010, t15015, t18324, t18658, t18663, t213, t257, t865);
    (t18725, t18726, t18729, t18730, t18738, t18742, t18746, t18750, t18761, t18784, t18785, t18791)
}
