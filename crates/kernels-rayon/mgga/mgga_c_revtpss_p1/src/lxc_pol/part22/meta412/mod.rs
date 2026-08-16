//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2015;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2016;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2017;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta412(t3920: f64, t5603: f64, t2435: f64, t5718: f64, t1893: f64, t2453: f64, t3908: f64, t1904: f64, t3895: f64, t2439: f64, t213: f64, t5710: f64, t10157: f64, t10160: f64, t10163: f64, t10166: f64, t10169: f64, t10176: f64, t1445: f64, t4071: f64, t4078: f64, t5715: f64, t5775: f64, t13750: f64, t14088: f64, t14279: f64, t1343: f64, t13664: f64, t13667: f64, t13669: f64, t13671: f64, t13673: f64, t13674: f64, t13682: f64, t13683: f64, t13716: f64, t13885: f64, t13886: f64, t13888: f64, t1450: f64, t198: f64, t3889: f64, t4135: f64, t4139: f64, t4144: f64, t532: f64, t5532: f64, t5541: f64, t5542: f64, t9524: f64, t9542: f64, t9854: f64, t9865: f64, t9868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14280, t14290, t14293, t14294, t14296, t14297, t14299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2015(t3920, t5603, t2435, t5718, t1893, t2453, t3908, t1904, t3895, t2439, t213, t5710);
        let t14302 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2016(t10157, t10160, t10163, t10166, t10169, t10176, t14280, t14290, t14294, t14297, t14299, t1445, t4071, t4078, t5715, t5775);
        let (t14304, t14308) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2017(t13750, t14088, t14279, t14302, t1343, t13664, t13667, t13669, t13671, t13673, t13674, t13682, t13683, t13716, t13885, t13886, t13888, t1450, t198, t3889, t4135, t4139, t4144, t532, t5532, t5541, t5542, t9524, t9542, t9854, t9865, t9868);
    (t14280, t14290, t14293, t14294, t14296, t14297, t14299, t14304, t14308)
}
