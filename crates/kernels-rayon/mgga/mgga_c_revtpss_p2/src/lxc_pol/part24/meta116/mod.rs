//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta116 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk649;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta116(t565: f64, t1466: f64, t602: f64, t1469: f64, t2275: f64, t2282: f64, t2299: f64, t2306: f64, t116: f64, t1501: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4146, t4147) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk649(t565);
        let (t4173, t4201, t4210, t4227, t4232, t4248) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk650(t1466, t602, t1469, t2275, t2282, t2299, t2306, t116, t1501);
    (t4146, t4147, t4173, t4201, t4210, t4227, t4232, t4248)
}
