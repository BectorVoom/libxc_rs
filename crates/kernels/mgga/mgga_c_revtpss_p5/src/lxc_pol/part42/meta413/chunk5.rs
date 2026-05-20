//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1462/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1462<F: Float>(t6781: F, t9593: F, t5537: F, t5591: F, t13643: F, t1448: F, t22205: F, t22206: F, t22207: F, t22208: F, t22209: F, t22211: F, t5536: F, t5541: F, t9421: F, t9427: F, t9429: F, t9514: F, t9517: F, t9521: F, t9546: F, t9569: F, t9574: F, t9577: F, t9588: F) -> F {
    let t22475 = t6781 * t9593;
    let t22479 = t5537 * t5591;
    let t22482 = F::new(2.0) * t1448 * t22475 * t5541 + F::new(12.0) * t22479 * t5536 - t13643 + t22205 + t22206 + t22207 + t22208 + t22209 - t22211 + t9421 - t9427 + t9429 + t9514 - t9517 - t9521 + t9546 + t9569 - t9574 - t9577 - t9588;
    t22482
}
