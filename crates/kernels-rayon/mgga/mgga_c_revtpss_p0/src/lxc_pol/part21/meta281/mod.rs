//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1512;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1513;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1514;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta281(t1445: f64, t3895: f64, t2439: f64, t1420: f64, t2453: f64, t3908: f64, t4067: f64, t786: f64, t1364: f64, t213: f64, t4066: f64, t1426: f64, t3917: f64, t10009: f64, t10147: f64, t10151: f64, t10154: f64, t10157: f64, t10160: f64, t1424: f64, t4071: f64, t4078: f64, t561: f64, t9691: f64, t9694: f64, t9695: f64, t9689: f64, t3889: f64, t566: f64, t1343: f64, t1353: f64, t1450: f64, t198: f64, t4139: f64, t4140: f64, t532: f64, t5536: f64, t9524: f64, t9542: f64, t9590: f64, t9593: f64, t9598: f64, t9599: f64, t9628: f64, t9854: f64, t9857: f64, t9859: f64, t9862: f64, t9865: f64, t9868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10162, t10163, t10165, t10166, t10168, t10169, t10171, t10174) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1512(t1445, t3895, t2439, t1420, t2453, t3908, t4067, t786, t1364, t213, t4066, t1426);
        let t10175 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1513(t10174, t786);
        let (t10176, t10178) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1514(t10175, t3917, t10009, t10147, t10151, t10154, t10157, t10160, t10163, t10166, t10169, t10171, t1424, t1445, t213, t4071, t4078, t561, t9691, t9694, t9695);
        let (t10179, t10190) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1515(t10178, t9689, t3889, t566, t1343, t1353, t1450, t198, t4139, t4140, t532, t5536, t9524, t9542, t9590, t9593, t9598, t9599, t9628, t9854, t9857, t9859, t9862, t9865, t9868);
    (t10162, t10163, t10165, t10166, t10168, t10169, t10171, t10174, t10175, t10176, t10179, t10190)
}
