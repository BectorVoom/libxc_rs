//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1617/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1617<F: Float>(t39442: F, t39483: F, t39520: F, t87303: F, t87304: F, t87305: F, t87306: F, t87307: F, t87309: F, t87312: F, t87314: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t87315: F, t87318: F) -> (F, F) {
    let t87635 = t39442 + t87303 + t87304 + t87305 + t87306 - t87307 + t87309 + t87312 - t39483 + t39520 + t87314;
    let t87637 = -t39528 + t39531 + t87315 + t39534 + t39537 - t39540 + t39741 + t39744 + t39747 + t87318 + t39750;
    (t87635, t87637)
}
