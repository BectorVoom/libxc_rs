//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1150;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1151;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1152;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1153;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1154;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta330(t45: f64, t190: f64, t22688: f64, t10439: f64, t4546: f64, t5966: f64, t18540: f64, t18545: f64, t18547: f64, t14363: f64, t22671: f64, t4328: f64, t5825: f64, t633: f64, t766: f64, zeta_threshold: f64, t57: f64, t4335: f64, t637: f64, t770: f64, t1544: f64, t18268: f64, t18850: f64, t198: f64, t23106: f64, t23110: f64, t23111: f64, t23114: f64, t2403: f64, t262: f64, t4541: f64, t765: f64, t9394: f64, t2723: f64, t6016: f64, t1558: f64, t5977: f64, t10871: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23121, t23123, t23124, t23127, t23128, t23129, t23130, t23138) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1150(t45, t190, t22688, t10439, t4546, t5966, t18540, t18545, t18547, t14363, t22671, t4328, t5825, t633, t766, zeta_threshold);
        let t23148 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1151(t57, t22671, t22688, t4335, t5825, t637, t770, t23138, zeta_threshold);
        let t23152 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1152(t1544, t18268, t18850, t198, t23106, t23110, t23111, t23114, t23123, t23124, t23127, t23128, t23129, t23130, t23148, t2403, t262, t4541, t765, t9394);
        let (t23160, t23167) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1153(t2723, t6016, t1558, t5977);
        let t23168 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1154(t10871, t23167);
        let t23172 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1155(t23167, t2723);
    (t23121, t23123, t23127, t23128, t23129, t23130, t23148, t23152, t23160, t23167, t23168, t23172)
}
