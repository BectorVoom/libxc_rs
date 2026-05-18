//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1246/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1246<F: Float>(t37393: F, t37398: F, t37401: F, t37407: F, t37413: F, t37415: F, t37419: F, t37423: F, t42840: F, t42843: F, t42845: F, t42850: F, t42854: F, t42858: F, t42860: F) -> F {
    let t43851 = -t42840 + t42843 - F::new(0.43368970657079495312e-4) * t37393 - t37398 + F::new(0.46116394948205481339e-3) * t37401 + t42845 + t37407 + t37413 - t37415 + t42850 - t42854 + F::new(0.14905073231436680509e-2) * t37419 + t42858 + F::new(0.36021158228745895953e-3) * t37423 - t42860;
    t43851
}
