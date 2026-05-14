//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 956/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk956<F: Float>(t492: F, t6547: F, t108: F, t6454: F, t379: F, t1564: F, t3238: F, t5743: F, t5498: F, t6414: F, t3266: F, t5675: F, t8411: F, t5674: F, t23057: F, t925: F) -> (F, F, F, F, F, F, F, F) {
    let t25856 = t6547 * t492;
    let t25861 = t6454 * t108;
    let t25862 = t25861 * t379;
    let t25863 = t1564 * t25862;
    let t25867 = t3238 * t5743;
    let t25869 = t6414 * t5498;
    let t25872 = t8411 * t5675 * t3266;
    let t25873 = t5674 * t25872;
    let t25875 = t1564 * t23057 * t925;
    (t25856, t25861, t25863, t25867, t25869, t25872, t25873, t25875)
}
