//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1150;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1151;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1152;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1153;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1154;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta330<F: Float>(t45: F, t190: F, t22688: F, t10439: F, t4546: F, t5966: F, t18540: F, t18545: F, t18547: F, t14363: F, t22671: F, t4328: F, t5825: F, t633: F, t766: F, zeta_threshold: F, t57: F, t4335: F, t637: F, t770: F, t1544: F, t18268: F, t18850: F, t198: F, t23106: F, t23110: F, t23111: F, t23114: F, t2403: F, t262: F, t4541: F, t765: F, t9394: F, t2723: F, t6016: F, t1558: F, t5977: F, t10871: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23121, t23123, t23124, t23127, t23128, t23129, t23130, t23138) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1150::<F>(t45, t190, t22688, t10439, t4546, t5966, t18540, t18545, t18547, t14363, t22671, t4328, t5825, t633, t766, zeta_threshold);
        let t23148 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1151::<F>(t57, t22671, t22688, t4335, t5825, t637, t770, t23138, zeta_threshold);
        let t23152 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1152::<F>(t1544, t18268, t18850, t198, t23106, t23110, t23111, t23114, t23123, t23124, t23127, t23128, t23129, t23130, t23148, t2403, t262, t4541, t765, t9394);
        let (t23160, t23167) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1153::<F>(t2723, t6016, t1558, t5977);
        let t23168 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1154::<F>(t10871, t23167);
        let t23172 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1155::<F>(t23167, t2723);
    (t23121, t23123, t23127, t23128, t23129, t23130, t23148, t23152, t23160, t23167, t23168, t23172)
}
