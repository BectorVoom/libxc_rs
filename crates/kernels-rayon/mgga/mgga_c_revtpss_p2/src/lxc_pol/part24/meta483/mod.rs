//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1474;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta483(t3671: f64, t371: f64, t6609: f64, t676: f64, t480: f64, t69637: f64, t17303: f64, t5323: f64, t5327: f64, t1284: f64, t20849: f64, t3624: f64, t3625: f64, t44250: f64, t6639: f64, t21439: f64, t11249: f64, t6622: f64, t3682: f64, t6667: f64, t474: f64, t6593: f64, t3089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70511, t70578, t70583, t70758, t70800) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1474(t3671, t371, t6609, t676, t480, t69637, t17303, t5323, t5327, t1284, t20849, t3624);
        let (t70809, t70819, t70890, t70942, t70994) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1475(t3625, t44250, t6639, t21439, t3624, t11249, t6622, t3682, t6667, t474, t6593, t3089);
    (t70511, t70578, t70583, t70758, t70800, t70809, t70819, t70890, t70942, t70994)
}
