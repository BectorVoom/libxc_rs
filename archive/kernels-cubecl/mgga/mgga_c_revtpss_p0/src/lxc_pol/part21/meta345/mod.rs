//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1676;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1677;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta345<F: Float>(t11610: F, t981: F, t11572: F, t300: F, t11467: F, t11506: F, t11509: F, t11114: F, t11118: F, t11530: F, t11533: F, t11547: F, t11596: F, t11600: F, t11604: F, t11608: F, t11594: F, t1045: F, t373: F, t1042: F, t1034: F, t360: F, t11244: F, t11240: F, t3154: F, t357: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11612, t11614, t11616, t11618, t11619) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1676::<F>(t11610, t981, t11572, t300, t11467, t11506, t11509, t11114, t11118, t11530, t11533, t11547, t11596, t11600, t11604, t11608);
        let t11620 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1677::<F>(t11594, t11619);
        let (t11622, t11623, t11626, t11627, t11629, t11630, t11631) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1678::<F>(t1045, t11620, t373, t1042, t1034, t360, t11244, t11240, t3154, t357);
    (t11612, t11614, t11616, t11618, t11620, t11622, t11623, t11626, t11627, t11629, t11630, t11631)
}
