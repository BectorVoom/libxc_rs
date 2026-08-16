//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1940;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta553(t225: f64, t29636: f64, t1949: f64, t6048: f64, t25317: f64, t6071: f64, t7071: f64, t233: f64, t1957: f64, t1558: f64, t231: f64, t7759: f64, t7076: f64, t1580: f64, t1956: f64, t213: f64, t25303: f64, t25307: f64, t257: f64, t27187: f64, t27189: f64, t27192: f64, t27196: f64, t27199: f64, t27203: f64, t27214: f64, t27217: f64, t29611: f64, t6049: f64, t6072: f64, t7053: f64, t7070: f64, t7766: f64, t7770: f64, t7779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29637, t29643, t29644, t29654, t29655, t29658, t29659, t29668) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1940(t225, t29636, t1949, t6048, t25317, t6071, t7071, t233, t1957, t1558, t231, t7759);
        let (t29669, t29672) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1941(t29668, t7076, t1580, t1956, t213, t25303, t25307, t257, t27187, t27189, t27192, t27196, t27199, t27203, t27214, t27217, t29611, t29637, t29644, t29655, t29659, t6049, t6072, t7053, t7070, t7766, t7770, t7779);
    (t29637, t29643, t29644, t29654, t29655, t29658, t29659, t29668, t29669, t29672)
}
