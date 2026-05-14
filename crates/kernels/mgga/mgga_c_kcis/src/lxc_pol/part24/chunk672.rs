//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 672/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk672<F: Float>(t8604: F, t97: F, t128: F, t66: F, t15: F, t736: F, t32: F, t5: F, t742: F, t103: F, t2357: F, t119: F, t681: F, t2368: F, t2376: F, t645: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8605 = t97 * t8604;
    let t8611 = t66 * t128;
    let t8618 = t736 * t15;
    let t8630 = 0.34451131037037037036e-2 * t5 * t742 * t32;
    let t8631 = t103 * t2357;
    let t8634 = t119 * t681;
    let t8637 = t103 * t2368;
    let t8640 = t103 * t2376;
    let t8643 = t66 * t645;
    (t8605, t8611, t8618, t8630, t8631, t8634, t8637, t8640, t8643)
}
