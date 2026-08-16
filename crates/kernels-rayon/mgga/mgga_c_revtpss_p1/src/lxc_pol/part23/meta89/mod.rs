//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta89 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk616;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta89(t159: f64, t215: f64, t10: f64, t17: f64, t576: f64, t580: f64, t15: f64, t22: f64, t11: f64, t14: f64, t584: f64, t588: f64, t20: f64, t27: f64, t12: f64, t19: f64, t592: f64, t596: f64, t21: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1941, t2219, t2221, t2223, t2224, t2226, t2228) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk616(t159, t215, t10, t17, t576, t580, t15, t22, t11, t14, t584, t588);
        let (t2230, t2231, t2233, t2235, t2236) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk617(t20, t27, t12, t19, t592, t596, t21);
    (t1941, t2219, t2221, t2223, t2224, t2226, t2228, t2230, t2231, t2233, t2235, t2236)
}
