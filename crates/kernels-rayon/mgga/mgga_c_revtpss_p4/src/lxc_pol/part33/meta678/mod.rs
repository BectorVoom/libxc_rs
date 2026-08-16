//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2210;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta678(t108710: f64, t1936: f64, t21881: f64, t93: f64, t30143: f64, t7002: f64, t27123: f64, t7741: f64, t28219: f64, t28042: f64, t7889: f64, t2322: f64, t30004: f64, t5523: f64, t27833: f64, t7935: f64, t1448: f64, t6922: f64, t28196: f64, t28197: f64, t28067: f64, t98450: f64, t7897: f64, t8995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109241, t109244, t109246, t109248, t109250, t109252, t109254) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2210(t108710, t1936, t21881, t93, t30143, t7002, t27123, t7741, t28219, t28042, t7889, t2322, t30004);
        let (t109256, t109262, t109266, t109268, t109269) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2211(t30004, t5523, t27833, t7935, t1448, t6922, t28196, t28197, t28067, t98450, t7897, t8995);
    (t109241, t109244, t109246, t109248, t109250, t109252, t109254, t109256, t109262, t109266, t109268, t109269)
}
