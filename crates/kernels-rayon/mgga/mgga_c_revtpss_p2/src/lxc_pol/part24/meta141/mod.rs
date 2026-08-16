//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk725;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk726;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta141(t213: f64, t5744: f64, t4086: f64, t1892: f64, t545: f64, t869: f64, t689: f64, t72: f64, t1432: f64, t686: f64, t1385: f64, t116: f64, t1518: f64, t2219: f64, t2221: f64, t2223: f64, t2226: f64, t2228: f64, t2230: f64, t2233: f64, t2235: f64, t2239: f64, t1497: f64, t1469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767, t5801) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk725(t213, t5744, t4086, t1892, t545, t869, t689, t72, t1432, t686, t1385, t116, t1518);
        let (t5812, t5816) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk726(t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t2239, t1497);
        let t5819 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk727(t1469);
    (t5745, t5755, t5759, t5760, t5761, t5763, t5765, t5767, t5801, t5812, t5816, t5819)
}
