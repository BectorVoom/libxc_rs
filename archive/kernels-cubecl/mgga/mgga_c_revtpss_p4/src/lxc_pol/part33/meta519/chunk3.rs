//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1861/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1861<F: Float>(t27883: F, t7063: F, t7286: F, t72: F, t7929: F, t686: F) -> (F, F, F, F) {
    let t27884 = t7063 * t27883;
    let t27885 = t27884 * t7286;
    let t27887 = t7929 * t72;
    let t27888 = t27887 * t686;
    (t27884, t27885, t27887, t27888)
}
