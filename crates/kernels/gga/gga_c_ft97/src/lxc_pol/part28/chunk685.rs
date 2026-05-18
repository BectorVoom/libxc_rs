//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 685/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk685<F: Float>(t167: F, t2185: F, t26950: F, t609: F, t6615: F, t574: F, t605: F, t1359: F, t3590: F, t1017: F, t5975: F, t26768: F) -> (F, F, F, F, F, F) {
    let t26952 = t2185 * t167 * t26950;
    let t26955 = t6615 * t609;
    let t26957 = t574 * t605 * t26955;
    let t26961 = t574 * t3590 * t1359;
    let t26965 = t574 * t5975 * t1017;
    let t26969 = t574 * t167 * t26768;
    (t26952, t26955, t26957, t26961, t26965, t26969)
}
