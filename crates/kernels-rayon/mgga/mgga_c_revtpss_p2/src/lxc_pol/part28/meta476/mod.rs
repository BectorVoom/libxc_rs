//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1803;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1804;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1805;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1806;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1807;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1808;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta476(t2411: f64, t7086: f64, t11064: f64, t1962: f64, t2408: f64, t30: f64, t605: f64, t890: f64, t2832: f64, t1940: f64, t1963: f64, t2257: f64, t2403: f64, t25198: f64, t25206: f64, t25208: f64, t25211: f64, t25215: f64, t25436: f64, t4541: f64, t7010: f64, t7087: f64, t7091: f64, t7092: f64, t1032: f64, t1071: f64, t7150: f64, t11120: f64, t359: f64, t1976: f64, t3270: f64, t1096: f64, t7135: f64, t7160: f64, t1982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t25440 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1803(t2411, t7086);
        let t25445 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1804(t11064, t1962);
        let (t25446, t25449, t25452, t25459) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1805(t2408, t30, t605, t890, t2832, t1940, t1963, t2257, t2403, t25198, t25206, t25208, t25211, t25215, t25436, t25440, t25445, t4541, t7010, t7087, t7091, t7092);
        let t25460 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1806(t1032, t1071);
        let t25461 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1807(t25460, t7150);
        let t25464 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1808(t11120, t359);
        let (t25466, t25470, t25473) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1809(t1976, t3270, t25464, t1096, t7135, t7160, t1982, t25460);
    (t25440, t25445, t25446, t25449, t25452, t25459, t25460, t25461, t25464, t25466, t25470, t25473)
}
