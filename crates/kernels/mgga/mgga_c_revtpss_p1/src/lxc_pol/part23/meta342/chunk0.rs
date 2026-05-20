//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1644/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1644<F: Float>(t2516: F, t4398: F, t2496: F, t2619: F, t4302: F, t4186: F, t750: F, t706: F, t4395: F, t4537: F, t892: F, t123: F, t1534: F) -> (F, F, F, F, F, F, F, F) {
    let t14334 = t4398 * t2516;
    let t14336 = t4398 * t2496;
    let t14339 = t4302 * t2619;
    let t14341 = t750 * t4186;
    let t14343 = F::new(8.0) * t706 * t14341;
    let t14345 = F::new(2.0) * t4395 * t750;
    let t14353 = t4537 * t892;
    let t14362 = t1534 * t123;
    (t14334, t14336, t14339, t14341, t14343, t14345, t14353, t14362)
}
