//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2451;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2452;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2453;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta585<F: Float>(t6017: F, t72: F, t686: F, t2798: F, t5978: F, t14568: F, t4500: F, t18699: F, t231: F, t2783: F, t2782: F, t18677: F, t18681: F, t2723: F, t4503: F, t10916: F, t14577: F, t14581: F, t14590: F, t14596: F, t14603: F, t14608: F, t14948: F, t6041: F, t874: F, t10661: F, t10923: F, t10925: F, t10939: F, t10948: F, t10964: F, t10966: F, t10969: F, t10971: F, t14546: F, t14951: F, t14972: F, t1559: F, t18525: F, t4366: F, t4504: F, t6022: F, t820: F, t18687: F, t18722: F, t868: F, t10503: F, t10507: F, t10511: F, t10984: F, t14998: F, t15004: F, t15006: F, t15010: F, t15015: F, t18324: F, t18658: F, t18663: F, t213: F, t257: F, t865: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18725, t18726, t18727, t18729, t18730, t18731, t18733, t18738, t18739, t18742) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2451::<F>(t6017, t72, t686, t2798, t5978, t14568, t4500, t18699, t231, t2783, t2782, t18677);
        let (t18746, t18750, t18754) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2452::<F>(t18742, t2782, t18681, t231, t2783, t18677, t2723, t4503, t10916, t14577, t14581, t14590, t14596, t14603, t14608, t14948, t18727, t18731, t18733, t18739);
        let (t18761, t18782) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2453::<F>(t6041, t72, t686, t874, t10661, t10923, t10925, t10939, t10948, t10964, t10966, t10969, t10971, t14546, t14951, t14972, t1559, t18525, t18677, t18681, t18699, t4366, t4504, t6022, t820);
        let (t18784, t18785, t18791) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2454::<F>(t18687, t18722, t18754, t18782, t868, t10503, t10507, t10511, t10984, t14998, t15004, t15006, t15010, t15015, t18324, t18658, t18663, t213, t257, t865);
    (t18725, t18726, t18729, t18730, t18738, t18742, t18746, t18750, t18761, t18784, t18785, t18791)
}
