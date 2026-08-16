//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta144 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk735;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta144(t5883: f64, t94: f64, t1518: f64, t1843: f64, t1513: f64, t2339: f64, t1504: f64, t2349: f64, t100: f64, t5823: f64, t1479: f64, t1509: f64, tau1: f64, t2357: f64, t108: f64, t105: f64, t109: f64, t1507: f64, t1510: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5884, t5887, t5891, t5892, t5895, t5896, t5899, t5902, t5907) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk735(t5883, t94, t1518, t1843, t1513, t2339, t1504, t2349, t100, t5823, t1479, t1509, tau1);
        let (t5908, t5911, t5912, t5915) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk736(t2357, t5907, t5823, t108, t105, t109, t1507, t1510, t5896, t5899, t5902, t97);
    (t5884, t5887, t5891, t5892, t5895, t5902, t5907, t5908, t5911, t5912, t5915)
}
