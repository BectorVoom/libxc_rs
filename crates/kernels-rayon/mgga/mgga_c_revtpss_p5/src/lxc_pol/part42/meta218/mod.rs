//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk848;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk849;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk850;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta218(t1868: f64, t4140: f64, t3854: f64, t3859: f64, t3862: f64, t3867: f64, t3871: f64, t3873: f64, t4030: f64, t4035: f64, t4037: f64, t4042: f64, t4139: f64, t5634: f64, t5637: f64, t5639: f64, t5640: f64, t5641: f64, t5782: f64, t118: f64, t1310: f64, t1315: f64, t1453: f64, t1502: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t2322: f64, t4246: f64, t4248: f64, t4254: f64, t4257: f64, t4293: f64, t4297: f64, t508: f64, t511: f64, t5517: f64, t5528: f64, t569: f64, t649: f64, t651: f64, t671: f64, t3: f64, t116: f64, t1518: f64, t670: f64, t117: f64, t4292: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, param_d: f64, t2219: f64, t2221: f64, t2223: f64, t2226: f64, t2228: f64, t2230: f64, t2233: f64, t2235: f64, t2239: f64, t1497: f64, t1469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5786 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk848(t1868, t4140, t3854, t3859, t3862, t3867, t3871, t3873, t4030, t4035, t4037, t4042, t4139, t5634, t5637, t5639, t5640, t5641);
        let (t5787, t5789) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk849(t5782, t5786, t118, t1310, t1315, t1453, t1502, t1519, t1843, t1847, t1911, t2322, t4246, t4248, t4254, t4257, t4293, t4297, t508, t511, t5517, t5528, t569, t649, t651, t671);
        let (t5790, t5795, t5801, t5802, t5805, t5808) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk850(t3, t5789, t116, t1518, t670, t117, t4292, t1459, t1461, t1916, t1918, t572, t573, param_d);
        let (t5812, t5816, t5819) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk851(t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t2239, t1497, t1469);
    (t5787, t5789, t5790, t5795, t5801, t5802, t5805, t5808, t5812, t5816, t5819)
}
