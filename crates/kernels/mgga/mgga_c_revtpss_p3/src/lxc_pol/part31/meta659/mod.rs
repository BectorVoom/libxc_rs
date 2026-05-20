//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta659 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2230;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2231;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2232;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2233;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta659<F: Float>(t13272: F, t1470: F, t28150: F, t7715: F, t29543: F, t644: F, t77: F, t1497: F, t7719: F, t1926: F, t29547: F, t1927: F, t5872: F, t2247: F, t5826: F, t60673: F, t6957: F, t101222: F, t101230: F, t101333: F, t10309: F, t25157: F, t25162: F, t25164: F, t28147: F, t28151: F, t28154: F, t34176: F, t6960: F, t5: F, t108768: F, t108799: F, t108829: F, t108854: F, t108889: F, t108931: F, t108963: F, t117: F, t27154: F, t98450: F, t28177: F, t7898: F, t28043: F, t4248: F, t651: F, t6765: F, t7002: F, t108716: F, t108718: F, t108721: F, t108723: F, t108725: F, t108727: F, t1310: F, t2007: F, t21814: F, t21891: F, t25805: F, t28025: F, t28030: F, t28050: F, t29569: F, t4297: F, t508: F, t5877: F, t5887: F, t6985: F, t7221: F, t7732: F, t28167: F, t86753: F, t8717: F, t13648: F, t2014: F, t7934: F, t29589: F, t7235: F, t13426: F, t7742: F, t18227: F, t28063: F) -> (F, F, F, F, F, F, F, F) {
        let (t108966, t108971, t108975, t108979, t108983, t108986) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2230::<F>(t13272, t1470, t28150, t7715, t29543, t644, t77, t1497, t7719, t1926, t29547, t1927, t5872);
        let t109001 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2231::<F>(t108986, t1926, t2247, t5826, t60673, t6957, t101222, t101230, t101333, t10309, t108966, t108971, t108975, t108979, t108983, t25157, t25162, t25164, t28147, t28151, t28154, t34176, t6960);
        let (t109006, t109012, t109014) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2232::<F>(t5, t108768, t108799, t108829, t108854, t108889, t108931, t108963, t109001, t117, t27154, t98450, t28177, t7898);
        let t109030 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2233::<F>(t28043, t4248, t651, t6765, t7002, t108716, t108718, t108721, t108723, t108725, t108727, t109006, t109012, t109014, t1310, t2007, t21814, t21891, t25805, t28025, t28030, t28050, t29569, t4297, t508, t5877, t5887, t6985, t7221, t7732);
        let (t109035, t109038, t109039, t109041, t109043, t109045) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2234::<F>(t28167, t86753, t8717, t13648, t2014, t7934, t29589, t7235, t13426, t7742, t18227, t28063, t4248);
    (t109006, t109030, t109035, t109038, t109039, t109041, t109043, t109045)
}
