//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 635/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk635<F: Float>(t1003: F, t7709: F, t5329: F, t1014: F, t2180: F, t283: F, t380: F) -> (F, F, F, F, F) {
    let t7710 = t7709 * t1003;
    let t7711 = t5329 * t7710;
    let t7716 = t1014 * t2180;
    let t7717 = 0.16581944444444444444e-2 * t7716;
    let t7718 = t380 * t283;
    (t7710, t7711, t7716, t7717, t7718)
}
