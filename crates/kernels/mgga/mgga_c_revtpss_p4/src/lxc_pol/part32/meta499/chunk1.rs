//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1783/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1783<F: Float>(t28799: F, t28822: F, t28861: F, t28923: F, t532: F, t1450: F, t5627: F, t9069: F, t26411: F, t7900: F, t28176: F, t7488: F) -> (F, F, F, F, F, F) {
    let t28925 = t28799 + t28822 + t28861 + t28923;
    let t28926 = t532 * t28925;
    let t28927 = t28926 * t1450;
    let t28929 = t9069 * t5627;
    let t28932 = t26411 * t7900;
    let t28935 = t7488 * t28176;
    (t28925, t28926, t28927, t28929, t28932, t28935)
}
