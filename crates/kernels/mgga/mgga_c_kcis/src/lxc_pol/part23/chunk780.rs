//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 780/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk780<F: Float>(t3728: F, t3739: F, t1466: F, t4108: F, t3735: F, t1017: F, t11670: F, t86: F, t4142: F, t4149: F, t2820: F, t3751: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t11780 = t3728 * t3739;
    let t11782 = t4108 * t1466;
    let t11783 = t11782 * sigma2;
    let t11799 = t3728 * t3735;
    let t11806 = t86 * t1017 * t11670;
    let t11811 = t4142 * t4149;
    let t11814 = t86 * t2820 * t3751;
    (t11780, t11782, t11783, t11799, t11806, t11811, t11814)
}
