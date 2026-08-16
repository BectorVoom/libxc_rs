//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1676;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1677;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta345(t11610: f64, t981: f64, t11572: f64, t300: f64, t11467: f64, t11506: f64, t11509: f64, t11114: f64, t11118: f64, t11530: f64, t11533: f64, t11547: f64, t11596: f64, t11600: f64, t11604: f64, t11608: f64, t11594: f64, t1045: f64, t373: f64, t1042: f64, t1034: f64, t360: f64, t11244: f64, t11240: f64, t3154: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11612, t11614, t11616, t11618, t11619) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1676(t11610, t981, t11572, t300, t11467, t11506, t11509, t11114, t11118, t11530, t11533, t11547, t11596, t11600, t11604, t11608);
        let t11620 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1677(t11594, t11619);
        let (t11622, t11623, t11626, t11627, t11629, t11630, t11631) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1678(t1045, t11620, t373, t1042, t1034, t360, t11244, t11240, t3154, t357);
    (t11612, t11614, t11616, t11618, t11620, t11622, t11623, t11626, t11627, t11629, t11630, t11631)
}
