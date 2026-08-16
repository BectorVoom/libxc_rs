//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2960/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2960<F: Float>(t78446: F, t78449: F, t78451: F, t78456: F, t78458: F, t78460: F, t78463: F, t78465: F, t78469: F, t78472: F, t78474: F, t24186: F, t3336: F) -> (F, F) {
    let t78475 = -t78446 + t78449 + t78451 - t78456 + t78458 + t78460 + t78463 - t78465 + t78469 - t78472 - t78474;
    let t78478 = t24186 * t3336;
    (t78475, t78478)
}
