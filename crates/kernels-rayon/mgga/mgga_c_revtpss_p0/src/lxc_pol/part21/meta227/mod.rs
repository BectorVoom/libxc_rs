//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta227 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1355;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1356;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1357;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1358;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1359;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta227(t198: f64, t530: f64, t1868: f64, t566: f64, t532: f64, t1907: f64, t4147: f64, t30: f64, t1317: f64, t1857: f64, t1320: f64, t1468: f64, t3833: f64, t2: f64, t513: f64, t580: f64, t605: f64, t1711: f64, t3841: f64, t516: f64, zeta_threshold: f64, t33: f64, t1113: f64, t162: f64, t189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5536 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1355(t198, t530);
        let (t5537, t5541) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1356(t1868, t566, t198, t532);
        let t5542 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1357(t1907, t4147);
        let (t5546, t5548, t5549, t5552, t5556, t5557, t5560) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1358(t30, t1317, t1857, t1320, t1468, t3833, t2, t513, t580, t605, t1711, t3841, t516, zeta_threshold);
        let t5566 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1359(t33, t1113, t5557, t5560, t580, t162, t5556, zeta_threshold);
        let t5567 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1360(t189, t5566);
    (t5536, t5537, t5541, t5542, t5546, t5548, t5549, t5552, t5557, t5560, t5566, t5567)
}
