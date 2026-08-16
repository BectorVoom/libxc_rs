//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1647;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta345(t14473: f64, t2439: f64, t212: f64, t4469: f64, t780: f64, t689: f64, t1579: f64, t2769: f64, t886: f64, t252: f64, t2782: f64, t2470: f64, t4480: f64, t2465: f64, t1558: f64, t836: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14474, t14476, t14477, t14479, t14480, t14481, t14482, t14484, t14485) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1647(t14473, t2439, t212, t4469, t780, t689, t1579, t2769, t886, t252, t2782, t2470, t4480);
        let (t14486, t14494, t14495) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1648(t14485, t2465, t1558, t836, t231);
    (t14474, t14476, t14477, t14479, t14480, t14481, t14482, t14484, t14485, t14486, t14494, t14495)
}
