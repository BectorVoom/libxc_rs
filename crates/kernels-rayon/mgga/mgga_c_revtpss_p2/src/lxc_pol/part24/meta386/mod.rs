//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta386(t1280: f64, t24633: f64, t1811: f64, t6628: f64, t3769: f64, t5464: f64, t6622: f64, t5332: f64, t1287: f64, t24739: f64, t24751: f64, t24704: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t24964, t24973, t24974, t24978, t24981, t24986, t24989) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1289(t1280, t24633, t1811, t6628, t3769, t5464, t6622, t5332, t1287, t24739, t24751, t24704);
    (t24964, t24973, t24974, t24978, t24981, t24986, t24989)
}
