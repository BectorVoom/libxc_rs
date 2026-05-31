//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2251/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2251<F: Float>(t108071: F, t108114: F, t108713: F, t109030: F, t109075: F, t109129: F, t109170: F, t109275: F, t6941: F, t7331: F, t5795: F, t7950: F) -> (F, F, F) {
    let t109278 = t108071 + t108114 + t108713 + t109030 + t109075 + t109129 + t109170 + t109275;
    let t109282 = F::cast_from(6.0_f64) * t6941 * t7331;
    let t109288 = F::cast_from(12.0_f64) * t5795 * t7950;
    (t109278, t109282, t109288)
}
