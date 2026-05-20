//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1564/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1564<F: Float>(t17605: F, t21090: F, t127: F, t12988: F, t24617: F, t371: F, t20842: F, t5323: F, t1010: F, t22700: F, t21169: F, t5373: F) -> (F, F, F, F, F) {
    let t83916 = t17605 * t21090;
    let t83920 = t12988 * t371 * t127 * t24617;
    let t83922 = t5323 * t20842;
    let t83962 = t22700 * t1010;
    let t83992 = t5373 * t21169;
    (t83916, t83920, t83922, t83962, t83992)
}
