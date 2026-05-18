//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1262/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1262<F: Float>(t17303: F, t7613: F, t26866: F, t5436: F, t17361: F, t7618: F, t17307: F, t2138: F, t3682: F, t8172: F, t3655: F, t8185: F) -> (F, F, F, F, F, F) {
    let t104825 = t7613 * t17303;
    let t104888 = t5436 * t26866;
    let t104905 = t7618 * t17361;
    let t104927 = t17307 * t2138;
    let t104963 = t8172 * t3682;
    let t104988 = t8185 * t3655;
    (t104825, t104888, t104905, t104927, t104963, t104988)
}
