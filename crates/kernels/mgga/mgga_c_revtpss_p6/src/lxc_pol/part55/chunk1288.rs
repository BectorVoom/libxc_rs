//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1288/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1288<F: Float>(t128517: F, t128519: F, t128521: F, t128528: F, t128531: F, t128533: F, t128535: F, t128537: F, t130929: F, t2163: F, t2322: F, t28683: F, t28935: F, t34824: F, t4254: F, t651: F, t671: F, t7683: F, t7983: F, t8764: F) -> F {
    let t131000 = -F::new(2.0) * t2163 * t28683 * t651 - F::new(2.0) * t651 * t7683 * t7983 - F::new(2.0) * t130929 * t671 - F::new(2.0) * t2322 * t34824 + F::new(3.0) * t28935 * t8764 - F::new(2.0) * t34824 * t4254 - t128517 - t128519 - t128521 + t128528 + t128531 - t128533 - t128535 - t128537;
    t131000
}
