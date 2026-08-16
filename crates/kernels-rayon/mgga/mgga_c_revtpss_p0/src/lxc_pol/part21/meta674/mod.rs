//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2475;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta674(t3494: f64, t3519: f64, t12258: f64, t698: f64, t13026: f64, t240: f64, t3361: f64, t1146: f64, t9303: f64, t12270: f64, t2304: f64, t2439: f64, t3424: f64, t3421: f64, t12278: f64, t12274: f64, t25273: f64, t268: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43752, t43762, t43764, t43766, t43771, t43773, t43776, t43781) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2475(t3494, t3519, t12258, t698, t13026, t240, t3361, t1146, t9303, t12270, t2304, t2439, t3424);
        let (t43783, t43785, t43787, t43813) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2476(t2439, t3421, t12278, t698, t12274, t25273, t268, t404);
    (t43752, t43762, t43764, t43766, t43771, t43773, t43776, t43781, t43783, t43785, t43787, t43813)
}
