//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1913/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1913<F: Float>(t7234: F, t8995: F, t14468: F, t30: F, t2: F, t2411: F, t580: F, t890: F, t892: F, t775: F, t1583: F, t2430: F) -> (F, F, F, F, F) {
    let t98588 = t7234 * t8995;
    let t98627 = t30 * t14468;
    let t98631 = t2411 * t2;
    let t98633 = t98631 * t580 * t890;
    let t98646 = t892 * t2;
    let t98648 = t98646 * t580 * t775;
    let t98651 = t1583 * t2430;
    (t98588, t98627, t98633, t98648, t98651)
}
