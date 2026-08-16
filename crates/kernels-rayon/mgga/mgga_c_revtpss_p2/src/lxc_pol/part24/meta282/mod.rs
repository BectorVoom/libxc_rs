//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta282(t2852: f64, t5825: f64, t11354: f64, t6113: f64, t11358: f64, t6132: f64, t698: f64, t6135: f64, t6138: f64, t300: f64, t6184: f64, t6104: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18936, t18979, t18987, t19002, t19004, t19009, t19049, t19056) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1058(t2852, t5825, t11354, t6113, t11358, t6132, t698, t6135, t6138, t300, t6184, t6104, t914);
    (t18936, t18979, t18987, t19002, t19004, t19009, t19049, t19056)
}
