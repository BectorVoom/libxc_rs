//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 896/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk896<F: Float>(t379: F, t422: F, t538: F, t129: F, t1691: F, t135: F, t22626: F, t14: F, t549: F, t72: F, t1696: F, t5551: F) -> (F, F, F, F, F, F, F) {
    let t23717 = t422 * t538 * t379;
    let t23721 = t129 * t1691;
    let t23722 = t22626 * t135;
    let t23723 = t23721 * t23722;
    let t23724 = t549 * t14;
    let t23725 = t23724 * t72;
    let t23728 = t5551 * t1696;
    (t23717, t23721, t23722, t23723, t23724, t23725, t23728)
}
