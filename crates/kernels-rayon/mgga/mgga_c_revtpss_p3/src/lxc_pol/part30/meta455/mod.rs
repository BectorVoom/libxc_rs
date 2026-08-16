//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1733;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1734;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1735;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta455(t17879: f64, t460: f64, t3584: f64, t5457: f64, t5351: f64, t1269: f64, t3766: f64, t1280: f64, t17345: f64, t1287: f64, t17389: f64, t17600: f64, t1248: f64, t5412: f64, t1204: f64, t12723: f64, t1281: f64, t1285: f64, t1288: f64, t12987: f64, t17289: f64, t17307: f64, t17861: f64, t17864: f64, t17869: f64, t17876: f64, t1825: f64, t3552: f64, t3666: f64, t3751: f64, t3755: f64, t3782: f64, t5449: f64, t5459: f64, t5466: f64, t5478: f64, t5481: f64, t5494: f64, t3568: f64, t5486: f64, t1794: f64, t3727: f64, t1770: f64, t3759: f64, t5245: f64, t13126: f64, t487: f64, t12050: f64, t3601: f64, t471: f64, t17710: f64, t5462: f64, t3754: f64, t5219: f64, t1234: f64, t12699: f64, t12709: f64, t12717: f64, t17331: f64, t1822: f64, t3670: f64, t3746: f64, t3756: f64, t3770: f64, t3774: f64, t3778: f64, t3787: f64, t490: f64, t5436: f64, t5446: f64, t5470: f64, t5491: f64, t17186: f64, t17859: f64, t1277: f64, t1828: f64, t3738: f64, t13182: f64, t3566: f64, t488: f64, t1276: f64, t1774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17880, t17883, t17884, t17888, t17893, t17902, t17905) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1732(t17879, t460, t3584, t5457, t5351, t1269, t3766, t1280, t17345, t1287, t17389, t17600);
        let t17912 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1733(t1248, t1287, t5412, t1204, t12723, t1281, t1285, t1288, t12987, t17289, t17307, t17861, t17864, t17869, t17876, t17880, t17884, t17888, t17893, t17902, t17905, t1825, t3552, t3666, t3751, t3755, t3782, t5449, t5459, t5466, t5478, t5481, t5494);
        let (t17917, t17921, t17934, t17941, t17944, t17945, t17948) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1734(t3568, t5486, t1287, t1794, t3727, t1770, t3766, t3759, t5245, t5457, t5351, t13126, t487);
        let (t17951, t17961) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1735(t17948, t460, t12050, t3601, t471, t17710, t1204, t5462, t3754, t5219, t1234, t12699, t12709, t12717, t12723, t1285, t17331, t1770, t17917, t17921, t17934, t17941, t17945, t1822, t3670, t3746, t3756, t3770, t3774, t3778, t3787, t490, t5436, t5446, t5466, t5470, t5491);
        let (t17963, t17964, t17968, t17973, t17974) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1736(t17186, t17859, t17912, t17961, t1277, t1828, t3738, t13182, t3566, t488, t1276, t1774);
    (t17883, t17944, t17951, t17963, t17964, t17968, t17973, t17974)
}
