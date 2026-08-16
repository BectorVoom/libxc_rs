//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3049/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3049<F: Float>(t14586: F, t2645: F, t10529: F, t2782: F, t10535: F, t136: F, t2457: F, t4424: F, t10523: F, t14568: F, t2482: F, t2801: F, t4423: F, t879: F) -> (F, F, F, F) {
    let t51608 = t14586 * t2645;
    let t51610 = t2782 * t10529 * t51608;
    let t51614 = t10535 * t4424 * t136 * t2457;
    let t51617 = t14568 * t10523;
    let t51621 = t2482 * t879 * t4423 * t2801;
    (t51610, t51614, t51617, t51621)
}
