//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk809;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta160(t1843: f64, t670: f64, t2616: f64, t2524: f64, t1534: f64, t72: f64, t757: f64, t1469: f64, t750: f64, t706: f64, t190: f64, t4186: f64, t1531: f64, t705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4297, t4300, t4301, t4302, t4303, t4304, t4305, t4306, t4307, t4308) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk809(t1843, t670, t2616, t2524, t1534, t72, t757, t1469, t750, t706, t190, t4186);
        let (t4310, t4311) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk810(t4308, t706, t1531, t705);
    (t4297, t4300, t4301, t4302, t4303, t4304, t4305, t4306, t4307, t4308, t4310, t4311)
}
