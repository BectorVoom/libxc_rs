//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 462/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk462<F: Float>(t2470: F, t788: F, t787: F, t206: F, t242: F, t240: F, t72: F, t2394: F, t828: F, t225: F, t786: F) -> (F, F, F, F, F, F, F) {
    let t2471 = t788 * t2470;
    let t2473 = F::new(0.13009920719177044025e-1) * t787 * t2471;
    let t2475 = F::new(1.0) / t242 / t206;
    let t2476 = t240 * t2475;
    let t2477 = t2476 * t72;
    let t2479 = t2477 * t828 * t2394;
    let t2482 = t786 * t225;
    (t2471, t2473, t2475, t2476, t2477, t2479, t2482)
}
