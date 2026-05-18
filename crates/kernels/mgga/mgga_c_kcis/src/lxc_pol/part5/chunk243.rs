//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 243/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk243<F: Float>(t228: F, t780: F, t827: F, t895: F, t899: F, t906: F, t279: F, sigma0: F) -> (F, F) {
    let t908 = t228 * t895 - t899 * t906 - t780 + t827;
    let t910 = F::new(1.0) / t279;
    let t911 = sigma0 * t910;
    (t908, t911)
}
