//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1574;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1575;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1576;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta310(t10811: f64, t2751: f64, t2681: f64, t820: f64, t823: f64, t839: f64, t2430: f64, t775: f64, t2477: f64, t828: f64, t222: f64, t9727: f64, t2737: f64, t9802: f64, t10639: f64, t827: f64, t221: f64, t2485: f64, t2754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10812, t10815) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1574(t10811, t2751, t2681, t820, t823);
        let (t10816, t10818) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1575(t10815, t839, t2430, t775);
        let (t10820, t10824, t10826, t10828, t10832) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1576(t10818, t2477, t828, t222, t9727, t2737, t9802, t10639, t827, t221, t2485, t2754);
    (t10812, t10815, t10816, t10818, t10820, t10824, t10826, t10828, t10832)
}
