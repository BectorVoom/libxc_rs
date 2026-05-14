//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 380/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk380<F: Float>(t60: F, t2398: F, t63: F, t697: F, t72: F, t700: F, t209: F, t2379: F) -> (F, F, F, F, F) {
    let t70 = 0.0 < t60;
    let t2399 = t63 * t2398;
    let t2403 = 1.0 / t697 / t72;
    let t2404 = t700 * t700;
    let t2406 = t209 * t2403 * t2404;
    let t2410 = piecewise3(t70, t2379, -t2379);
    (t2399, t2403, t2404, t2406, t2410)
}
