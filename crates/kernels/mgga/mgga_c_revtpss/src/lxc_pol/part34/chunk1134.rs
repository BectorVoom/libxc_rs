//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1134/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1134<F: Float>(t26054: F, t5722: F, t1955: F, t7283: F, t72: F, t7920: F, t686: F, t25895: F, t25878: F, t1426: F, t27836: F, t7063: F) -> (F, F, F, F, F, F, F, F) {
    let t27861 = t26054 * t5722;
    let t27868 = t1955 * t7283;
    let t27872 = t7920 * t72;
    let t27873 = t27872 * t686;
    let t27874 = t25895 * t27873;
    let t27876 = t25878 * t27873;
    let t27883 = t27836 * t1426;
    let t27884 = t7063 * t27883;
    (t27861, t27868, t27872, t27873, t27874, t27876, t27883, t27884)
}
