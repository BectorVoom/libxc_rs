//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 504/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk504<F: Float>(t1626: F, t964: F, t1633: F, t3014: F, t300: F, t2986: F, t1646: F, t993: F) -> (F, F, F, F, F) {
    let t4685 = t1626 * t964;
    let t4711 = t1633 * t3014;
    let t4719 = t300 * t1626;
    let t4724 = t2986 * t1633;
    let t4746 = t1646 * t993;
    (t4685, t4711, t4719, t4724, t4746)
}
