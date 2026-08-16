//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1469;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta480(t3451: f64, t6481: f64, t12555: f64, t6534: f64, t3565: f64, t6563: f64, t225: f64, t1261: f64, t12879: f64, t247: f64, t6429: f64, t11262: f64, t1247: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t69488, t69511, t69636, t69637, t69661, t69668) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1469(t3451, t6481, t12555, t6534, t3565, t6563, t225, t1261, t12879, t247, t6429, t11262, t1247, t6624);
    (t69488, t69511, t69636, t69637, t69661, t69668)
}
