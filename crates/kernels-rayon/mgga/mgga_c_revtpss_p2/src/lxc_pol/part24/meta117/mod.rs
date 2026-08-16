//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk651;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta117(t1514: f64, t625: f64, t1513: f64, t2339: f64, t1504: f64, t2349: f64, t1509: f64, t2357: f64, t1534: f64, t72: f64, t757: f64, t1469: f64, t750: f64, t706: f64, t1531: f64, t705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4261, t4263, t4269, t4279, t4302, t4303, t4305) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk651(t1514, t625, t1513, t2339, t1504, t2349, t1509, t2357, t1534, t72, t757, t1469, t750);
        let (t4306, t4311) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk652(t4305, t706, t1531, t705);
    (t4261, t4263, t4269, t4279, t4302, t4303, t4305, t4306, t4311)
}
