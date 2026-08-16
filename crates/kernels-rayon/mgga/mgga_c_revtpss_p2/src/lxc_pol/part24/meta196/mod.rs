//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk927;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk928;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk929;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta196(t2482: f64, t27: f64, t4000: f64, t555: f64, t5744: f64, t786: f64, t4083: f64, t9303: f64, t123: f64, t212: f64, t2434: f64, t138: f64, t2438: f64, t785: f64, t9990: f64, t1432: f64, t1433: f64, t9288: f64, t225: f64, t9646: f64, t1428: f64, t22: f64, t2452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10001, t10022, t10023, t10035, t10069) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk927(t2482, t27, t4000, t555, t5744, t786, t4083, t9303, t123, t212, t2434);
        let t10073 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk928(t138, t2438, t785);
        let (t10090, t10102, t10111) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk929(t555, t9990, t1432, t1433, t9288, t225, t9646);
        let (t10114, t10115) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk930(t10111, t1428, t22, t2452);
    (t10001, t10022, t10023, t10035, t10069, t10073, t10090, t10102, t10111, t10114, t10115)
}
