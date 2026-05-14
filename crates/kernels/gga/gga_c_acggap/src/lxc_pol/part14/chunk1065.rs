//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1065/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1065<F: Float>(t615: F, t6413: F, t119: F, t150: F, t187: F, t31905: F, t33489: F, t33511: F, t33516: F, t33518: F, t33523: F, t33525: F, t33529: F, t33533: F, t33538: F, t33541: F, t33546: F, t33554: F, t40595: F, t621: F, t7931: F, t8440: F) -> (F,) {
    let t40601 = t615 * t6413;
    let t40604 = -0.17347256376410398924e1 * t33511 + t33516 - 0.17347256376410398924e1 * t33518 + t33523 - 0.17347256376410398924e1 * t33525 - t33529 - 0.17347256376410398924e1 * t7931 * t33489 * t8440 + 0.65854491829355115987e0 * t119 * t40595 * t150 * t187 + t33533 + t33538 - t33541 - 0.17347256376410398924e1 * t31905 + t33546 - 0.4336814094102599731e0 * t40601 * t621 + t33554;
    (t40604,)
}
