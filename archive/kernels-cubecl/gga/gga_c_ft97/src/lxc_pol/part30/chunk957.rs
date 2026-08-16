//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 957/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk957<F: Float>(t668: F, t7546: F, t33701: F, t681: F, t89: F, t1882: F, t33609: F, t33622: F, t33626: F, t33646: F, t33779: F, t33730: F) -> (F, F, F, F, F, F, F, F) {
    let t142347 = t7546 * t668;
    let t142365 = t89 * t681 * t33701;
    let t142382 = t1882 * t33609;
    let t142393 = t1882 * t33622;
    let t142395 = t1882 * t33626;
    let t142404 = t1882 * t33646;
    let t142410 = t1882 * t33779;
    let t142412 = t1882 * t33730;
    (t142347, t142365, t142382, t142393, t142395, t142404, t142410, t142412)
}
