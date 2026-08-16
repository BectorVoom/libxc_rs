//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2230;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2231;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2232;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2233;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta659(t13272: f64, t1470: f64, t28150: f64, t7715: f64, t29543: f64, t644: f64, t77: f64, t1497: f64, t7719: f64, t1926: f64, t29547: f64, t1927: f64, t5872: f64, t2247: f64, t5826: f64, t60673: f64, t6957: f64, t101222: f64, t101230: f64, t101333: f64, t10309: f64, t25157: f64, t25162: f64, t25164: f64, t28147: f64, t28151: f64, t28154: f64, t34176: f64, t6960: f64, t5: f64, t108768: f64, t108799: f64, t108829: f64, t108854: f64, t108889: f64, t108931: f64, t108963: f64, t117: f64, t27154: f64, t98450: f64, t28177: f64, t7898: f64, t28043: f64, t4248: f64, t651: f64, t6765: f64, t7002: f64, t108716: f64, t108718: f64, t108721: f64, t108723: f64, t108725: f64, t108727: f64, t1310: f64, t2007: f64, t21814: f64, t21891: f64, t25805: f64, t28025: f64, t28030: f64, t28050: f64, t29569: f64, t4297: f64, t508: f64, t5877: f64, t5887: f64, t6985: f64, t7221: f64, t7732: f64, t28167: f64, t86753: f64, t8717: f64, t13648: f64, t2014: f64, t7934: f64, t29589: f64, t7235: f64, t13426: f64, t7742: f64, t18227: f64, t28063: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108966, t108971, t108975, t108979, t108983, t108986) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2230(t13272, t1470, t28150, t7715, t29543, t644, t77, t1497, t7719, t1926, t29547, t1927, t5872);
        let t109001 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2231(t108986, t1926, t2247, t5826, t60673, t6957, t101222, t101230, t101333, t10309, t108966, t108971, t108975, t108979, t108983, t25157, t25162, t25164, t28147, t28151, t28154, t34176, t6960);
        let (t109006, t109012, t109014) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2232(t5, t108768, t108799, t108829, t108854, t108889, t108931, t108963, t109001, t117, t27154, t98450, t28177, t7898);
        let t109030 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2233(t28043, t4248, t651, t6765, t7002, t108716, t108718, t108721, t108723, t108725, t108727, t109006, t109012, t109014, t1310, t2007, t21814, t21891, t25805, t28025, t28030, t28050, t29569, t4297, t508, t5877, t5887, t6985, t7221, t7732);
        let (t109035, t109038, t109039, t109041, t109043, t109045) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2234(t28167, t86753, t8717, t13648, t2014, t7934, t29589, t7235, t13426, t7742, t18227, t28063, t4248);
    (t109006, t109030, t109035, t109038, t109039, t109041, t109043, t109045)
}
