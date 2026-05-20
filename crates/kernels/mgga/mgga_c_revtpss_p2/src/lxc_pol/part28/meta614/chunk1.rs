//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2146/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2146<F: Float>(t25207: F, t98674: F, t1940: F, t1963: F, t2403: F, t25198: F, t25206: F, t25208: F, t25449: F, t27158: F, t27160: F, t27169: F, t27364: F, t27368: F, t27395: F, t4541: F, t605: F, t7087: F, t7783: F, t98627: F, t98635: F, t98637: F, t98650: F, t98652: F, t98659: F, t98662: F, t98669: F) -> F {
    let t98675 = t25207 * t98674;
    let t98678 = F::new(3.0) / F::new(2.0) * t2403 * t1963 * t98627 - t98635 - F::new(3.0) * t98637 * t25208 + F::new(3.0) * t4541 * t7783 * t25198 + F::new(3.0) * t2403 * t7087 * t27395 + t98650 - F::new(3.0) / F::new(2.0) * t25206 * t98652 + F::new(3.0) * t2403 * t7087 * t27169 - F::new(3.0) * t25206 * t98659 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t98662 - t1940 * t27368 * t25449 + F::new(6.0) * t98669 * t27160 + t1940 * t27364 * t605 - F::new(6.0) * t27158 * t98675;
    t98678
}
