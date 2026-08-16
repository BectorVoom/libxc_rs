//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1358;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1359;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1360;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1361;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta375(t40360: f64, t839: f64, t10639: f64, t221: f64, t2484: f64, t2485: f64, t10820: f64, t2652: f64, t231: f64, t40262: f64, t10841: f64, t10845: f64, t10878: f64, t2741: f64, t2722: f64, t853: f64, t10726: f64, t10786: f64, t2661: f64, t10943: f64, t2663: f64, t2645: f64, t2662: f64, t2749: f64, t2721: f64, t40324: f64, t40326: f64, t40333: f64, t40337: f64, t40340: f64, t40345: f64, t40349: f64, t40355: f64, t40357: f64, t825: f64, t827: f64, t828: f64, t10815: f64, t2648: f64, t2756: f64, t2681: f64, t2719: f64, t820: f64, t2726: f64, t10850: f64, t10861: f64, t10111: f64, t823: f64, t9720: f64, t685: f64, t837: f64, t10837: f64, t9775: f64, t10828: f64, t10818: f64, t10703: f64, t2674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40361, t40365, t40367, t40369, t40374) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1358(t40360, t839, t10639, t221, t2484, t2485, t10820, t2652, t231, t40262, t10841, t10845);
        let (t40376, t40378, t40381, t40385, t40390) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1359(t10878, t2741, t2722, t853, t10726, t10786, t2661, t10943, t2663, t2645, t2662, t2749);
        let t40392 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1360(t2721, t40324, t40326, t40333, t40337, t40340, t40345, t40349, t40355, t40357, t40361, t40365, t40367, t40369, t40374, t40376, t40381, t40385, t40390, t825, t827, t828);
        let (t40393, t40395, t40399, t40403, t40406) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1361(t10815, t2648, t2756, t2681, t2719, t820, t2726, t10850, t10861, t221, t2485, t10111, t823, t9720);
        let (t40409, t40411, t40413, t40421) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1362(t40406, t685, t827, t837, t10837, t9775, t10828, t2741, t10818, t221, t10703, t2674);
    (t40369, t40378, t40392, t40393, t40395, t40399, t40403, t40409, t40411, t40413, t40421)
}
