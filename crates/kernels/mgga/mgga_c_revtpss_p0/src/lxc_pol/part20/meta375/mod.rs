//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta375 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1358;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1359;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1360;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1361;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta375<F: Float>(t40360: F, t839: F, t10639: F, t221: F, t2484: F, t2485: F, t10820: F, t2652: F, t231: F, t40262: F, t10841: F, t10845: F, t10878: F, t2741: F, t2722: F, t853: F, t10726: F, t10786: F, t2661: F, t10943: F, t2663: F, t2645: F, t2662: F, t2749: F, t2721: F, t40324: F, t40326: F, t40333: F, t40337: F, t40340: F, t40345: F, t40349: F, t40355: F, t40357: F, t825: F, t827: F, t828: F, t10815: F, t2648: F, t2756: F, t2681: F, t2719: F, t820: F, t2726: F, t10850: F, t10861: F, t10111: F, t823: F, t9720: F, t685: F, t837: F, t10837: F, t9775: F, t10828: F, t10818: F, t10703: F, t2674: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40361, t40365, t40367, t40369, t40374) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1358::<F>(t40360, t839, t10639, t221, t2484, t2485, t10820, t2652, t231, t40262, t10841, t10845);
        let (t40376, t40378, t40381, t40385, t40390) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1359::<F>(t10878, t2741, t2722, t853, t10726, t10786, t2661, t10943, t2663, t2645, t2662, t2749);
        let t40392 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1360::<F>(t2721, t40324, t40326, t40333, t40337, t40340, t40345, t40349, t40355, t40357, t40361, t40365, t40367, t40369, t40374, t40376, t40381, t40385, t40390, t825, t827, t828);
        let (t40393, t40395, t40399, t40403, t40406) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1361::<F>(t10815, t2648, t2756, t2681, t2719, t820, t2726, t10850, t10861, t221, t2485, t10111, t823, t9720);
        let (t40409, t40411, t40413, t40421) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1362::<F>(t40406, t685, t827, t837, t10837, t9775, t10828, t2741, t10818, t221, t10703, t2674);
    (t40369, t40378, t40392, t40393, t40395, t40399, t40403, t40409, t40411, t40413, t40421)
}
