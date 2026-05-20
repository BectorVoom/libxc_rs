//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1512;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1513;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1514;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1515;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta281<F: Float>(t1445: F, t3895: F, t2439: F, t1420: F, t2453: F, t3908: F, t4067: F, t786: F, t1364: F, t213: F, t4066: F, t1426: F, t3917: F, t10009: F, t10147: F, t10151: F, t10154: F, t10157: F, t10160: F, t1424: F, t4071: F, t4078: F, t561: F, t9691: F, t9694: F, t9695: F, t9689: F, t3889: F, t566: F, t1343: F, t1353: F, t1450: F, t198: F, t4139: F, t4140: F, t532: F, t5536: F, t9524: F, t9542: F, t9590: F, t9593: F, t9598: F, t9599: F, t9628: F, t9854: F, t9857: F, t9859: F, t9862: F, t9865: F, t9868: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10162, t10163, t10165, t10166, t10168, t10169, t10171, t10174) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1512::<F>(t1445, t3895, t2439, t1420, t2453, t3908, t4067, t786, t1364, t213, t4066, t1426);
        let t10175 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1513::<F>(t10174, t786);
        let (t10176, t10178) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1514::<F>(t10175, t3917, t10009, t10147, t10151, t10154, t10157, t10160, t10163, t10166, t10169, t10171, t1424, t1445, t213, t4071, t4078, t561, t9691, t9694, t9695);
        let (t10179, t10190) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1515::<F>(t10178, t9689, t3889, t566, t1343, t1353, t1450, t198, t4139, t4140, t532, t5536, t9524, t9542, t9590, t9593, t9598, t9599, t9628, t9854, t9857, t9859, t9862, t9865, t9868);
    (t10162, t10163, t10165, t10166, t10168, t10169, t10171, t10174, t10175, t10176, t10179, t10190)
}
