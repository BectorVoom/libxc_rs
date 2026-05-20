//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta455 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1733;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1734;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1735;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta455<F: Float>(t17879: F, t460: F, t3584: F, t5457: F, t5351: F, t1269: F, t3766: F, t1280: F, t17345: F, t1287: F, t17389: F, t17600: F, t1248: F, t5412: F, t1204: F, t12723: F, t1281: F, t1285: F, t1288: F, t12987: F, t17289: F, t17307: F, t17861: F, t17864: F, t17869: F, t17876: F, t1825: F, t3552: F, t3666: F, t3751: F, t3755: F, t3782: F, t5449: F, t5459: F, t5466: F, t5478: F, t5481: F, t5494: F, t3568: F, t5486: F, t1794: F, t3727: F, t1770: F, t3759: F, t5245: F, t13126: F, t487: F, t12050: F, t3601: F, t471: F, t17710: F, t5462: F, t3754: F, t5219: F, t1234: F, t12699: F, t12709: F, t12717: F, t17331: F, t1822: F, t3670: F, t3746: F, t3756: F, t3770: F, t3774: F, t3778: F, t3787: F, t490: F, t5436: F, t5446: F, t5470: F, t5491: F, t17186: F, t17859: F, t1277: F, t1828: F, t3738: F, t13182: F, t3566: F, t488: F, t1276: F, t1774: F) -> (F, F, F, F, F, F, F, F) {
        let (t17880, t17883, t17884, t17888, t17893, t17902, t17905) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1732::<F>(t17879, t460, t3584, t5457, t5351, t1269, t3766, t1280, t17345, t1287, t17389, t17600);
        let t17912 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1733::<F>(t1248, t1287, t5412, t1204, t12723, t1281, t1285, t1288, t12987, t17289, t17307, t17861, t17864, t17869, t17876, t17880, t17884, t17888, t17893, t17902, t17905, t1825, t3552, t3666, t3751, t3755, t3782, t5449, t5459, t5466, t5478, t5481, t5494);
        let (t17917, t17921, t17934, t17941, t17944, t17945, t17948) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1734::<F>(t3568, t5486, t1287, t1794, t3727, t1770, t3766, t3759, t5245, t5457, t5351, t13126, t487);
        let (t17951, t17961) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1735::<F>(t17948, t460, t12050, t3601, t471, t17710, t1204, t5462, t3754, t5219, t1234, t12699, t12709, t12717, t12723, t1285, t17331, t1770, t17917, t17921, t17934, t17941, t17945, t1822, t3670, t3746, t3756, t3770, t3774, t3778, t3787, t490, t5436, t5446, t5466, t5470, t5491);
        let (t17963, t17964, t17968, t17973, t17974) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1736::<F>(t17186, t17859, t17912, t17961, t1277, t1828, t3738, t13182, t3566, t488, t1276, t1774);
    (t17883, t17944, t17951, t17963, t17964, t17968, t17973, t17974)
}
