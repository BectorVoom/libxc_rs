//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk845;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk846;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk847;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta218<F: Float>(t1868: F, t4140: F, t3854: F, t3859: F, t3862: F, t3867: F, t3871: F, t3873: F, t4030: F, t4035: F, t4037: F, t4042: F, t4139: F, t5634: F, t5637: F, t5639: F, t5640: F, t5641: F, t5782: F, t118: F, t1310: F, t1315: F, t1453: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t2322: F, t4246: F, t4248: F, t4254: F, t4257: F, t4293: F, t4297: F, t508: F, t511: F, t5517: F, t5528: F, t569: F, t649: F, t651: F, t671: F, t3: F, t116: F, t1518: F, t670: F, t117: F, t4292: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, param_d: F, t2219: F, t2221: F, t2223: F, t2226: F, t2228: F, t2230: F, t2233: F, t2235: F, t2239: F, t1497: F, t1469: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t5786 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk845::<F>(t1868, t4140, t3854, t3859, t3862, t3867, t3871, t3873, t4030, t4035, t4037, t4042, t4139, t5634, t5637, t5639, t5640, t5641);
        let (t5787, t5789) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk846::<F>(t5782, t5786, t118, t1310, t1315, t1453, t1502, t1519, t1843, t1847, t1911, t2322, t4246, t4248, t4254, t4257, t4293, t4297, t508, t511, t5517, t5528, t569, t649, t651, t671);
        let (t5790, t5795, t5801, t5802, t5805, t5808) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk847::<F>(t3, t5789, t116, t1518, t670, t117, t4292, t1459, t1461, t1916, t1918, t572, t573, param_d);
        let (t5812, t5816, t5819) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk848::<F>(t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t2239, t1497, t1469);
    (t5787, t5789, t5790, t5795, t5801, t5802, t5805, t5808, t5812, t5816, t5819)
}
