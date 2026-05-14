//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1053/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1053<F: Float>(t3334: F, t90: F, t7851: F, t871: F, t29896: F, t29898: F, t29901: F, t29911: F, t29913: F, t29915: F, t31610: F, t31612: F) -> (F,) {
    let t31614 = 4.0 / 3.0 * t3334 * t90;
    let t31615 = t7851 * t871;
    let t31617 = 63.0 / 512.0 * t29896;
    let t31618 = 385.0 / 16384.0 * t29898;
    let t31619 = 147.0 / 1048576.0 * t29901;
    let t31620 = 49.0 / 1048576.0 * t29911;
    let t31621 = 385.0 / 49152.0 * t29913;
    let t31622 = 21.0 / 512.0 * t29915;
    let t31623 = t31610 - t31612 + t31614 + t31615 / 2.0 + t31617 - t31618 + t31619 - t31620 + t31621 - t31622;
    (t31623,)
}
